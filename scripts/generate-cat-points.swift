// Optional asset authoring tool: run from the repository root on macOS with Swift.
// The site and normal Rust/Trunk build do not require Swift or AppKit.
import AppKit
import Foundation

let source = URL(fileURLWithPath: "assets/images/sleeping-cat.png")
guard let bitmap = NSBitmapImageRep(data: try Data(contentsOf: source)) else {
    fatalError("Cannot read the supplied cat PNG")
}
let spacing = 9
// Overlap the 9px grid enough to cover gaps between neighboring circles.
let radius = 6.4
var points: [[Int]] = []
for y in stride(from: spacing / 2, to: bitmap.pixelsHigh, by: spacing) {
    for x in stride(from: spacing / 2, to: bitmap.pixelsWide, by: spacing) {
        // Ignore transparent background and faint cutout fringes.
        guard let color = bitmap.colorAt(x: x, y: y)?.usingColorSpace(.deviceRGB),
              color.alphaComponent >= 0.9 else { continue }
        points.append([x, y, Int((color.redComponent * 255).rounded()),
                       Int((color.greenComponent * 255).rounded()),
                       Int((color.blueComponent * 255).rounded())])
    }
}
// Array order matches SVG circle order. Coordinates remain in original image space.
let model: [String: Any] = [
    "width": bitmap.pixelsWide, "height": bitmap.pixelsHigh,
    "spacing": spacing, "radius": radius,
    "pointFields": ["x", "y", "r", "g", "b"], "points": points
]
let json = try JSONSerialization.data(withJSONObject: model, options: [.sortedKeys])
try json.write(to: URL(fileURLWithPath: "assets/images/sleeping-cat-points.json"))
var svg = """
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 \(bitmap.pixelsWide) \(bitmap.pixelsHigh)" role="img" aria-labelledby="title desc">
<title id="title">Sleeping cat made of colored points</title>
<desc id="desc">A black and white cat curled up asleep, sampled from the supplied illustration.</desc>
<g stroke="none">

"""
for p in points {
    let color = String(format: "#%02x%02x%02x", p[2], p[3], p[4])
    svg += "<circle cx=\"\(p[0])\" cy=\"\(p[1])\" r=\"\(radius)\" fill=\"\(color)\"/>\n"
}
svg += "</g>\n</svg>\n"
try svg.write(toFile: "assets/images/sleeping-cat-points.svg", atomically: true, encoding: .utf8)
print("Generated \(points.count) colored points, SVG, and matching JSON.")
