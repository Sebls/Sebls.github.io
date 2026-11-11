"""Verify generated documents and local references. Run after the complete build."""
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlsplit, unquote
import sys

root = Path(sys.argv[1] if len(sys.argv) > 1 else 'dist')
base = sys.argv[2] if len(sys.argv) > 2 else '/'

class Document(HTMLParser):
    def __init__(self, text):
        super().__init__()
        self.refs, self.ids, self.mains, self.h1s = [], set(), 0, 0
        self.feed(text)
    def handle_starttag(self, tag, attrs):
        a = dict(attrs)
        if 'id' in a:
            assert a['id'] not in self.ids, f"duplicate id {a['id']}"
            self.ids.add(a['id'])
        self.mains += tag == 'main'
        self.h1s += tag == 'h1'
        for key in ['href', 'src']:
            if key in a:
                self.refs.append(a[key])

files = list(root.rglob('*.html'))
for page in ['index.html','about/index.html','experience/index.html','work/index.html','hobbies/index.html','gallery/index.html','blog/index.html','404.html']:
    assert (root/page).is_file(), page
for file in files:
    text = file.read_text()
    doc = Document(text)
    assert doc.mains == doc.h1s == 1, file
    assert '<title>' in text and 'og:title' in text, file
    assert 'data-base="' + base + '"' in text, file
    for ref in doc.refs:
        url = urlsplit(ref)
        if url.scheme or url.netloc:
            continue
        if not url.path:
            assert not url.fragment or unquote(url.fragment) in doc.ids, (file, ref)
            continue
        if url.path.startswith('/'):
            assert url.path.startswith(base), (file, ref, base)
            target = root/unquote(url.path[len(base):])
        else:
            target = file.parent/unquote(url.path)
        if target.is_dir():
            target = target/'index.html'
        assert target.is_file(), (file, ref, target)
for forbidden in ['docs','content','src','.git']:
    assert not (root/forbidden).exists(), forbidden
assert not list(root.rglob('*.md'))
assert not list(root.rglob('*design*.json'))
print(f'Checked {len(files)} HTML documents, local links, section targets, metadata, and source exclusion at {base}')
