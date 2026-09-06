#!/usr/bin/env python3
"""
scripts/sync_cv.py
Modular synchronization script for Sebls.github.io:
1. Extracts canonical CV data & compiles PDFs directly from the `sebls_cv` Git submodule.
2. Merges modular web-only content (volunteering, achievements, extra projects) from `data/cv_extra.json`.
3. Generates clean assets/data/cv-data.json and assets/js/cv-data.js for the website.
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent.parent
SUBMODULE_DIR = ROOT_DIR / "sebls_cv"
EXTRA_DATA_PATH = ROOT_DIR / "data" / "cv_extra.json"
ASSETS_DIR = ROOT_DIR / "assets"
DATA_DIR = ASSETS_DIR / "data"
PDF_DIR = ASSETS_DIR / "pdf"
JS_DIR = ASSETS_DIR / "js"


def run_cmd(cmd: list[str], cwd: Path | None = None) -> tuple[int, str, str]:
    try:
        proc = subprocess.run(
            cmd,
            cwd=str(cwd or ROOT_DIR),
            capture_output=True,
            text=True,
            check=False,
        )
        return proc.returncode, proc.stdout, proc.stderr
    except FileNotFoundError:
        return -1, "", f"Command not found: {cmd[0]}"


def typst_ast_to_html(node: any) -> str:
    """Recursively converts a Typst AST node (from typst eval) into clean semantic HTML."""
    if node is None:
        return ""
    if isinstance(node, str):
        return node
    if isinstance(node, (int, float)):
        return str(node)
    if isinstance(node, list):
        return "".join(typst_ast_to_html(x) for x in node)
    if isinstance(node, dict):
        func = node.get("func")
        if func == "text":
            return node.get("text", "")
        elif func == "space":
            return " "
        elif func == "strong":
            inner = typst_ast_to_html(node.get("body"))
            return f"<strong>{inner}</strong>"
        elif func == "emph":
            inner = typst_ast_to_html(node.get("body"))
            return f"<em>{inner}</em>"
        elif func == "sequence":
            return "".join(typst_ast_to_html(c) for c in node.get("children", []))
        elif func == "link":
            dest = node.get("dest", "")
            body = typst_ast_to_html(node.get("body")) or dest
            return f'<a href="{dest}" target="_blank" rel="noopener noreferrer">{body}</a>'
        elif "children" in node:
            return "".join(typst_ast_to_html(c) for c in node["children"])
        elif "body" in node:
            return typst_ast_to_html(node["body"])
        elif "text" in node:
            return node["text"]
    return str(node)


def eval_typst_content(lang: str) -> dict | None:
    """Evaluates content.typ in submodule via `typst eval` and returns structured dictionary."""
    content_file = f"cv/{lang}/content.typ"
    if not (SUBMODULE_DIR / content_file).exists():
        print(f"Warning: {content_file} not found in submodule.")
        return None

    expr = (
        f'import "{content_file}": *; '
        '(personal: personal, profile: profile, education: education, '
        'experience: experience, projects: projects, skills: skills)'
    )
    code, stdout, stderr = run_cmd(
        ["typst", "eval", "--root", "sebls_cv", expr],
        cwd=ROOT_DIR,
    )
    if code != 0:
        print(f"Error running typst eval on {content_file}: {stderr}")
        return None

    try:
        return json.loads(stdout)
    except Exception as e:
        print(f"Error parsing typst eval JSON: {e}")
        return None


def parse_typst_fallback(content_path: Path) -> dict:
    """Pure-Python regex parser for content.typ in case Typst CLI is unavailable."""
    text = content_path.read_text(encoding="utf-8")

    def typst_markup_to_html(s: str) -> str:
        s = re.sub(r"\*([^\*]+)\*", r"<strong>\1</strong>", s)
        s = re.sub(r"_([^_]+)_", r"<em>\1</em>", s)
        s = re.sub(r"\[(.*?)\]", r"\1", s)
        return s.strip()

    # Personal info
    personal = {}
    m_pers = re.search(r"#let\s+personal\s*=\s*\((.*?)\)", text, re.DOTALL)
    if m_pers:
        for k, v in re.findall(r'(\w+)\s*:\s*"([^"]*)"', m_pers.group(1)):
            personal[k] = v

    # Profile
    profile_html = ""
    m_prof = re.search(r"#let\s+profile\s*=\s*\[(.*?)\]", text, re.DOTALL)
    if m_prof:
        profile_html = typst_markup_to_html(m_prof.group(1))

    # Education
    education = []
    m_edu = re.search(r"#let\s+education\s*=\s*\((.*?)\n\)", text, re.DOTALL)
    if m_edu:
        blocks = re.findall(r"\((.*?)\),?", m_edu.group(1), re.DOTALL)
        for b in blocks:
            inst = re.search(r'institution\s*:\s*"([^"]*)"', b)
            deg = re.search(r"degree\s*:\s*\[?(.*?)\]?,?\s*\n", b)
            dates = re.search(r'dates\s*:\s*"([^"]*)"', b)
            education.append({
                "institution": inst.group(1) if inst else "",
                "degree": typst_markup_to_html(deg.group(1)) if deg else "",
                "dates": dates.group(1) if dates else "",
                "gpa": None,
            })

    # Experience
    experience = []
    m_exp = re.search(r"#let\s+experience\s*=\s*\((.*?)\n\)", text, re.DOTALL)
    if m_exp:
        blocks = re.findall(r"\((.*?bullets:\s*\(.*?\)\s*,?\s*)\)", m_exp.group(1), re.DOTALL)
        for b in blocks:
            role = re.search(r'role\s*:\s*"([^"]*)"', b)
            comp = re.search(r'company\s*:\s*"([^"]*)"', b)
            dates = re.search(r'dates\s*:\s*"([^"]*)"', b)
            loc = re.search(r'location\s*:\s*"([^"]*)"', b)
            contract = re.search(r'contract\s*:\s*"([^"]*)"', b)
            bullets_match = re.search(r"bullets:\s*\((.*?)\)", b, re.DOTALL)
            bullets = []
            if bullets_match:
                raw_bullets = re.findall(r"\[(.*?)\]", bullets_match.group(1), re.DOTALL)
                bullets = [typst_markup_to_html(rb) for rb in raw_bullets]
            experience.append({
                "role": role.group(1) if role else "",
                "company": comp.group(1) if comp else "",
                "dates": dates.group(1) if dates else "",
                "location": loc.group(1) if loc else "",
                "contract": contract.group(1) if contract else "",
                "bullets": bullets,
            })

    # Projects
    projects = []
    m_proj = re.search(r"#let\s+projects\s*=\s*\((.*?)\n\)", text, re.DOTALL)
    if m_proj:
        blocks = re.findall(r"\((.*?bullets:\s*\(.*?\)\s*,?\s*)\)", m_proj.group(1), re.DOTALL)
        for b in blocks:
            name = re.search(r'name\s*:\s*"([^"]*)"', b)
            role = re.search(r'role\s*:\s*"([^"]*)"', b)
            dates = re.search(r'dates\s*:\s*"([^"]*)"', b)
            url = re.search(r'url\s*:\s*"([^"]*)"', b)
            bullets_match = re.search(r"bullets:\s*\((.*?)\)", b, re.DOTALL)
            bullets = []
            if bullets_match:
                raw_bullets = re.findall(r"\[(.*?)\]", bullets_match.group(1), re.DOTALL)
                bullets = [typst_markup_to_html(rb) for rb in raw_bullets]
            projects.append({
                "name": name.group(1) if name else "",
                "role": role.group(1) if role else "",
                "dates": dates.group(1) if dates else "",
                "url": url.group(1) if url else "",
                "bullets": bullets,
            })

    # Skills
    skills = []
    m_skills = re.search(r"#let\s+skills\s*=\s*\((.*?)\n\)", text, re.DOTALL)
    if m_skills:
        for cat, items in re.findall(r'\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)', m_skills.group(1)):
            skills.append([cat, items])

    return {
        "personal": personal,
        "profile": profile_html,
        "education": education,
        "experience": experience,
        "projects": projects,
        "skills": skills,
    }


def extract_canonical_cv(lang: str) -> dict:
    """Extracts strictly canonical CV data from the submodule without any synthetic fallbacks."""
    raw = eval_typst_content(lang)
    if not raw:
        print(f"Using pure-Python parser for submodule cv/{lang}/content.typ...")
        raw = parse_typst_fallback(SUBMODULE_DIR / "cv" / lang / "content.typ")

    personal = raw.get("personal", {})
    profile = typst_ast_to_html(raw.get("profile", "")).strip()

    education = []
    for edu in raw.get("education", []):
        degree_html = typst_ast_to_html(edu.get("degree", "")).strip()
        education.append({
            "school": edu.get("institution", ""),
            "degree": degree_html,
            "date": edu.get("dates", ""),
            "gpa": edu.get("gpa"),
        })

    employment = []
    for exp in raw.get("experience", []):
        bullets = [typst_ast_to_html(b).strip() for b in exp.get("bullets", [])]
        role = exp.get("role", "")
        loc = exp.get("location", "")
        contract = exp.get("contract", "")
        
        role_parts = [role]
        if loc:
            role_parts.append(loc)
        role_str = " – ".join(role_parts)
        if contract:
            role_str += f" ({contract})"

        employment.append({
            "company": exp.get("company", ""),
            "role": role,
            "role_display": role_str,
            "date": exp.get("dates", ""),
            "location": loc,
            "contract": contract,
            "tasks": bullets,
        })

    projects = []
    for proj in raw.get("projects", []):
        bullets = [typst_ast_to_html(b).strip() for b in proj.get("bullets", [])]
        projects.append({
            "title": proj.get("name", ""),
            "role": proj.get("role", ""),
            "date": proj.get("dates", ""),
            "url": proj.get("url", ""),
            "subheader": proj.get("role", ""),
            "tasks": bullets,
        })

    skills = []
    for item in raw.get("skills", []):
        if isinstance(item, (list, tuple)) and len(item) == 2:
            cat, lst = item
            skills.append({
                "category": f"{cat}:",
                "list": lst,
            })

    return {
        "personal": personal,
        "profile": {"text": profile},
        "education": education,
        "employment": employment,
        "projects": projects,
        "skills": skills,
    }


def load_extra_content() -> dict:
    """Loads modular web-only CV presentation data from data/cv_extra.json."""
    if EXTRA_DATA_PATH.exists():
        try:
            with open(EXTRA_DATA_PATH, "r", encoding="utf-8") as f:
                return json.load(f)
        except Exception as e:
            print(f"⚠️ Error reading {EXTRA_DATA_PATH}: {e}")
    return {}


def compile_pdfs():
    """Compiles Typst CV documents if typst CLI is available."""
    if not (SUBMODULE_DIR / "cv").exists():
        print("Submodule cv directory not found; skipping PDF compilation.")
        return

    for lang in ("en", "fr"):
        typ_file = SUBMODULE_DIR / "cv" / lang / "cv.typ"
        pdf_file = SUBMODULE_DIR / "cv" / lang / "cv.pdf"
        if typ_file.exists():
            code, stdout, stderr = run_cmd(
                ["typst", "compile", "--root", str(SUBMODULE_DIR), str(typ_file), str(pdf_file)],
                cwd=SUBMODULE_DIR,
            )
            if code == 0:
                print(f"✅ Compiled {pdf_file.relative_to(ROOT_DIR)}")
            else:
                print(f"⚠️ Notice: typst compile skipped or failed for {lang}: {stderr.strip()}")

    # Ensure PDFs are also mirrored in assets/pdf/
    PDF_DIR.mkdir(parents=True, exist_ok=True)
    en_pdf = SUBMODULE_DIR / "cv" / "en" / "cv.pdf"
    fr_pdf = SUBMODULE_DIR / "cv" / "fr" / "cv.pdf"
    if en_pdf.exists():
        shutil.copy2(en_pdf, PDF_DIR / "cv_en.pdf")
        print(f"✅ Copied to {PDF_DIR / 'cv_en.pdf'}")
    if fr_pdf.exists():
        shutil.copy2(fr_pdf, PDF_DIR / "cv_fr.pdf")
        print(f"✅ Copied to {PDF_DIR / 'cv_fr.pdf'}")


def main():
    print("🚀 Synchronizing CV data from submodule & modular extra content...")

    # Ensure submodule directory exists
    if not SUBMODULE_DIR.exists():
        print("Initializing git submodule...")
        run_cmd(["git", "submodule", "update", "--init", "--recursive"])

    # 1. Compile PDFs from submodule
    compile_pdfs()

    # 2. Extract canonical data strictly from submodule
    canonical_en = extract_canonical_cv("en")
    canonical_fr = extract_canonical_cv("fr")

    # 3. Load modular extra content
    extra_data = load_extra_content()

    # 4. Construct English dataset
    extra_en = extra_data.get("en", {})
    en_projects = list(canonical_en["projects"])
    for ep in extra_en.get("extra_projects", []):
        if not any(p["title"] == ep["title"] for p in en_projects):
            en_projects.append(ep)

    en_dataset = {
        "personal": canonical_en["personal"],
        "subtitle": extra_en.get("subtitle", canonical_en["personal"].get("title", "AI Engineer")),
        "pdf_path": "sebls_cv/cv/en/cv.pdf",
        "nav": extra_en.get("nav", {
            "profile": "Profile",
            "education": "Education",
            "experience": "Experience",
            "skills": "Skills",
            "projects": "Projects",
            "achievements": "Achievements",
            "volunteering": "Volunteering",
            "hobbies": "Hobbies",
            "pdf": "PDF Version",
        }),
        "section": extra_en.get("section", {
            "profile": "Profile",
            "education": "Education",
            "employment": "Employment History",
            "skills": "Skills",
            "projects": "Projects",
            "achievements": "Achievements",
            "volunteering": "Volunteering",
            "hobbies": "Hobbies",
        }),
        "profile": canonical_en["profile"],
        "education": canonical_en["education"],
        "employment": canonical_en["employment"],
        "projects": en_projects,
        "skills": canonical_en["skills"],
        "achievements": extra_en.get("achievements", []),
        "volunteering": extra_en.get("volunteering", []),
        "hobbies": extra_en.get("hobbies", []),
    }

    # 5. Construct French dataset
    extra_fr = extra_data.get("fr", {})
    fr_projects = list(canonical_fr["projects"])
    for ep in extra_fr.get("extra_projects", []):
        if not any(p["title"] == ep["title"] for p in fr_projects):
            fr_projects.append(ep)

    fr_dataset = {
        "personal": canonical_fr["personal"],
        "subtitle": extra_fr.get("subtitle", "Étudiant Ingénieur en Double Diplôme à Mines Paris PSL et Universidad Nacional de Colombia"),
        "pdf_path": "sebls_cv/cv/fr/cv.pdf",
        "nav": extra_fr.get("nav", {
            "profile": "Profil",
            "education": "Formation",
            "experience": "Expérience",
            "skills": "Compétences",
            "projects": "Projets",
            "achievements": "Réalisations",
            "volunteering": "Bénévolat",
            "hobbies": "Loisirs",
            "pdf": "Version PDF",
        }),
        "section": extra_fr.get("section", {
            "profile": "Profil",
            "education": "Formation",
            "employment": "Expérience Professionnelle",
            "skills": "Compétences",
            "projects": "Projets",
            "achievements": "Réalisations",
            "volunteering": "Bénévolat",
            "hobbies": "Loisirs",
        }),
        "profile": canonical_fr["profile"],
        "education": canonical_fr["education"],
        "employment": canonical_fr["employment"],
        "projects": fr_projects,
        "skills": canonical_fr["skills"],
        "achievements": extra_fr.get("achievements", []),
        "volunteering": extra_fr.get("volunteering", []),
        "hobbies": extra_fr.get("hobbies", []),
    }

    # 6. Construct Spanish dataset (canonical submodule base mapped to Spanish + modular extra content)
    extra_es = extra_data.get("es", {})
    es_projects = [
        {
            "title": proj["title"],
            "role": "Proyecto Personal" if "personal" in proj.get("role", "").lower() else proj.get("role", ""),
            "date": proj.get("date", ""),
            "url": proj.get("url", ""),
            "subheader": proj.get("subheader", proj.get("role", "")),
            "tasks": [
                "Desarrollé un juego diario de adivinanza de palabras de código abierto con puntuación de <strong>similitud semántica</strong> aplicando <strong>embeddings basados en transformers</strong>.",
                "Implementé una arquitectura full-stack limpia guiada por el dominio utilizando Next.js (TypeScript) en el frontend y FastAPI (Python) en el backend, con pruebas automatizadas y documentación clara."
            ] if proj["title"] == "PhraseCrack" else proj["tasks"]
        }
        for proj in canonical_en["projects"]
    ]
    for ep in extra_es.get("extra_projects", []):
        if not any(p["title"] == ep["title"] for p in es_projects):
            es_projects.append(ep)

    es_dataset = {
        "personal": canonical_en["personal"],
        "subtitle": extra_es.get("subtitle", "Estudiante de Doble Titulación en Mines Paris PSL y Universidad Nacional de Colombia"),
        "pdf_path": "sebls_cv/cv/en/cv.pdf",
        "nav": extra_es.get("nav", {
            "profile": "Perfil",
            "education": "Educación",
            "experience": "Experiencia",
            "skills": "Habilidades",
            "projects": "Proyectos",
            "achievements": "Logros",
            "volunteering": "Voluntariado",
            "hobbies": "Aficiones",
            "pdf": "Versión PDF",
        }),
        "section": extra_es.get("section", {
            "profile": "Perfil",
            "education": "Educación",
            "employment": "Historial Laboral",
            "skills": "Habilidades",
            "projects": "Proyectos",
            "achievements": "Logros",
            "volunteering": "Voluntariado",
            "hobbies": "Aficiones",
        }),
        "profile": {
            "text": extra_es.get(
                "profile_text",
                "<strong>Ingeniero de IA</strong>, con sólida experiencia en ingeniería de software y el <strong>despliegue en producción de servicios de IA</strong> para aplicaciones de texto, imágenes y <strong>multimodales</strong>. Particularmente interesado en <strong>sistemas de LLMs</strong>, <strong>plataformas de IA escalables</strong> y sistemas inteligentes de nivel de producción."
            )
        },
        "education": [
            {
                "school": "École des Mines de Paris - PSL",
                "degree": "<strong>Título de Ingeniero</strong> (M.Sc.)",
                "date": "2025 – Previsto 2027",
                "gpa": None,
            },
            {
                "school": "Universidad Nacional de Colombia",
                "degree": "<strong>Ingeniería de Sistemas</strong> (B.Sc.) (Promedio: 4.74/5.0)",
                "date": "2021 – Previsto 2027",
                "gpa": None,
            }
        ],
        "employment": [
            {
                "company": "Engin AI",
                "role": "Ingeniero de IA",
                "role_display": "Ingeniero de IA – Miami, Estados Unidos (Contratista Independiente)",
                "date": "Ene 2026 – May 2026",
                "location": "Miami, Estados Unidos",
                "contract": "Contratista Independiente",
                "tasks": [
                    "Diseñé e implementé un <strong>agente de IA</strong> para <strong>clasificación de deepfakes</strong>, orquestando <strong>inferencia</strong> a través de modelos de clasificación privados.",
                    "Desarrollé la <strong>arquitectura del agente</strong> y el ecosistema de herramientas, integrando memoria a largo plazo basada en <strong>OpenKB</strong>, memoria conversacional a corto plazo y flujos de trabajo estructurados de interacción con modelos.",
                    "Diseñé y ejecuté <strong>evaluaciones en producción</strong> para medir el rendimiento de clasificación, la toma de decisiones del agente y la confiabilidad en escenarios <strong>multimodales</strong> de detección de deepfakes.",
                ]
            },
            {
                "company": "Engin AI",
                "role": "Ingeniero de Machine Learning",
                "role_display": "Ingeniero de Machine Learning – Miami, Estados Unidos (Contratista Independiente)",
                "date": "Nov 2025 – Ene 2026",
                "location": "Miami, Estados Unidos",
                "contract": "Contratista Independiente",
                "tasks": [
                    "Diseñé y desplegué más de 5 <strong>endpoints de inferencia multimodal</strong> (audio, imagen, video) listos para producción, procesando ~4,000 solicitudes diarias con una latencia promedio de 126 ms.",
                    "Extendí arquitecturas de <strong>deep learning</strong> para <strong>detección de audio</strong>, procesando más de 1 millón de muestras y mejorando la reproducibilidad de experimentos en un 40%.",
                    "Construí <strong>pipelines de datos escalables</strong> para datasets superiores a 100 GB, reduciendo el tiempo de despliegue en un 60% mediante validaciones automatizadas.",
                ]
            },
            {
                "company": "Engin AI",
                "role": "Ingeniero de Software",
                "role_display": "Ingeniero de Software – Miami, Estados Unidos (Contratista Independiente)",
                "date": "Sep 2025 – Nov 2025",
                "location": "Miami, Estados Unidos",
                "contract": "Contratista Independiente",
                "tasks": [
                    "Reduje la latencia de <strong>pipelines de ML</strong> en 70–80% mediante el rediseño de flujos de preprocesamiento de datos y optimización de recursos en contenedores.",
                    "Automaticé la validación y depuración de anotaciones (<strong>YOLO/COCO</strong>) con scripts en Python, eliminando el 75% de pasos manuales de QA.",
                ]
            },
            {
                "company": "Dataconstructors AI",
                "role": "Ingeniero de Software Junior",
                "role_display": "Ingeniero de Software Junior – Bogotá, Colombia",
                "date": "Mar 2025 – Sep 2025",
                "location": "Bogotá, Colombia",
                "contract": "",
                "tasks": [
                    "Reduje el tiempo de procesamiento geoespacial de una semana a tres horas diseñando orquestación automatizada de pipelines en Python interconectados con bases de datos.",
                    "Diseñé <strong>pipelines asistidos por IA</strong> para documentación técnica estructurada y validación de datos.",
                    "Desarrollé scripts en Python integrando <strong>LLMs</strong> para <strong>extracción semántica</strong> y validación automática de campos críticos, disminuyendo errores operativos en un 60%.",
                    "Desarrollé y mantuve servicios backend integrando APIs externas y bases de datos relacionales (MySQL/PostgreSQL) aplicando arquitectura limpia.",
                ]
            },
        ],
        "projects": es_projects,
        "skills": [
            {"category": "Lenguajes de Programación:", "list": "Python • TypeScript • C/C++"},
            {"category": "Frameworks y Librerías:", "list": "FastAPI • React • Next.js • Pandas • NumPy • PyTorch • TensorFlow"},
            {"category": "Bases de Datos:", "list": "SQL (PostgreSQL, SQLite) • NoSQL (Redis, MongoDB)"},
            {"category": "Infraestructura y Sistemas:", "list": "Linux • Git • GitHub Actions • Testing • CI/CD • Docker"},
            {"category": "Cloud:", "list": "Google Cloud Platform (Compute Engine, Cloud Storage, Vertex AI)"},
            {"category": "Inteligencia Artificial:", "list": "LLM (OpenAI API, OpenRouter, Claude Platform) • Computer Vision • Deep Learning • Evaluación de modelos"},
            {"category": "Idiomas:", "list": "Español (C2 - Nativo) • Inglés (C1) • Francés (C1) • Portugués (A2)"},
        ],
        "achievements": extra_es.get("achievements", []),
        "volunteering": extra_es.get("volunteering", []),
        "hobbies": extra_es.get("hobbies", []),
    }

    translations = {
        "en": en_dataset,
        "fr": fr_dataset,
        "es": es_dataset,
    }

    # 7. Write JSON and JS output files
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    json_path = DATA_DIR / "cv-data.json"
    js_path = JS_DIR / "cv-data.js"

    with open(json_path, "w", encoding="utf-8") as f:
        json.dump(translations, f, indent=2, ensure_ascii=False)
    print(f"✅ Generated {json_path.relative_to(ROOT_DIR)}")

    js_content = (
        "// Auto-generated by scripts/sync_cv.py — DO NOT EDIT DIRECTLY\n"
        f"const cvTranslations = {json.dumps(translations, indent=2, ensure_ascii=False)};\n\n"
        "if (typeof window !== 'undefined') {\n"
        "    window.cvTranslations = cvTranslations;\n"
        "}\n"
        "if (typeof module !== 'undefined' && module.exports) {\n"
        "    module.exports = cvTranslations;\n"
        "}\n"
    )
    with open(js_path, "w", encoding="utf-8") as f:
        f.write(js_content)
    print(f"✅ Generated {js_path.relative_to(ROOT_DIR)}")


if __name__ == "__main__":
    main()
