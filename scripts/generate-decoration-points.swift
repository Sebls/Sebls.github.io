// Optional macOS authoring tool. Run from the repository root with Swift.
// Rebuilds plants and flag only; originals, cat, and sleep marks are untouched.
import AppKit
import Foundation

struct Cell {
    var count = 0
    var x = 0.0, y = 0.0, red = 0.0, green = 0.0, blue = 0.0
}

let selected = CommandLine.arguments.dropFirst().first ?? "all"
guard ["all", "plants", "flag"].contains(selected) else {
    fatalError("Use all, plants, or flag")
}
let names = (selected == "flag" ? [] : (1...5).map { "plant-\($0)" })
    + (selected == "plants" ? [] : ["colombia-flag"])
let targetPitch = 310.0 * 9.0 / 1254.0 // Current desktop cat pitch, CSS pixels.

for name in names {
    let plant = name.hasPrefix("plant-")
    let stem = "assets/images/\(name)"
    let source = stem + (plant ? "-cutout.png" : ".png")
    guard let bitmap = NSBitmapImageRep(data: try Data(contentsOf: URL(fileURLWithPath: source))) else {
        fatalError("Cannot read \(source)")
    }
    let sw = bitmap.pixelsWide, sh = bitmap.pixelsHigh
    var mask = [Bool](repeating: false, count: sw * sh)
    for y in 0..<sh {
        for x in 0..<sw {
            mask[y * sw + x] = (bitmap.colorAt(x: x, y: y)?.alphaComponent ?? 0) >= 0.9
        }
    }
    // Keep the largest 8-connected source component for plants, before coarse
    // sampling, so thin stems survive but disconnected cutout dust does not.
    if plant {
        var visited = [Bool](repeating: false, count: sw * sh)
        var largest: [Int] = []
        for seed in mask.indices where mask[seed] && !visited[seed] {
            var queue = [seed], cursor = 0
            visited[seed] = true
            while cursor < queue.count {
                let index = queue[cursor], x = index % sw, y = index / sw
                cursor += 1
                for ny in max(0, y - 1)...min(sh - 1, y + 1) {
                    for nx in max(0, x - 1)...min(sw - 1, x + 1) {
                        let neighbor = ny * sw + nx
                        if mask[neighbor] && !visited[neighbor] {
                            visited[neighbor] = true
                            queue.append(neighbor)
                        }
                    }
                }
            }
            if queue.count > largest.count { largest = queue }
        }
        mask = [Bool](repeating: false, count: sw * sh)
        for index in largest { mask[index] = true }
    }
    let occupied = mask.indices.filter { mask[$0] }
    guard !occupied.isEmpty else { fatalError("Empty source: \(source)") }
    // Retain existing viewBoxes and CSS proportions. Trim plants consistently
    // with the original generator; the flag keeps its original coordinate space.
    let minX = plant ? occupied.map { $0 % sw }.min()! : 0
    let minY = plant ? occupied.map { $0 / sw }.min()! : 0
    let maxX = plant ? occupied.map { $0 % sw }.max()! : sw - 1
    let maxY = plant ? occupied.map { $0 / sw }.max()! : sh - 1
    let sourceScale = plant ? 1236.0 / Double(max(maxX - minX + 1, maxY - minY + 1)) : 1
    let width = plant ? Int(ceil(Double(maxX - minX + 1) * sourceScale)) + 18 : sw
    let height = plant ? Int(ceil(Double(maxY - minY + 1) * sourceScale)) + 18 : sh
    let displayScale = plant ? 63.0 / Double(max(width, height)) : 70.0 / Double(width)
    let spacing = targetPitch / displayScale
    let radius = spacing * (6.4 / 9.0)
    let columns = Int(ceil(Double(width) / spacing))
    var cells: [Int: Cell] = [:]
    for index in occupied {
        let sx = index % sw, sy = index / sw
        let x = Double(sx - minX) * sourceScale + (plant ? 9 : 0)
        let y = Double(sy - minY) * sourceScale + (plant ? 9 : 0)
        guard let color = bitmap.colorAt(x: sx, y: sy)?.usingColorSpace(.deviceRGB) else { continue }
        let key = Int(y / spacing) * columns + Int(x / spacing)
        var cell = cells[key, default: Cell()]
        cell.count += 1
        cell.x += x; cell.y += y
        cell.red += color.redComponent * 255
        cell.green += color.greenComponent * 255
        cell.blue += color.blueComponent * 255
        cells[key] = cell
    }
    // One dot per occupied cell, including sparse edge/stem cells. The opaque
    // pixel centroid follows narrow shapes; averaged opaque RGB removes noise.
    // Clamp centers so larger dots remain fully inside the existing viewBox.
    let points: [[Double]] = cells.keys.sorted().map { key in
        let cell = cells[key]!, count = Double(cell.count)
        return [max(radius, min(Double(width) - radius, cell.x / count)),
                max(radius, min(Double(height) - radius, cell.y / count)),
                (cell.red / count).rounded(), (cell.green / count).rounded(),
                (cell.blue / count).rounded()]
    }
    let model: [String: Any] = ["width": width, "height": height,
        "spacing": spacing, "radius": radius, "sampling": "opaque-cell-centroid",
        "targetPitchCssPx": targetPitch, "referenceDisplayLongestEdgeCssPx": plant ? 63 : 70,
        "pointFields": ["x", "y", "r", "g", "b"], "points": points]
    try JSONSerialization.data(withJSONObject: model, options: [.sortedKeys])
        .write(to: URL(fileURLWithPath: stem + "-points.json"))
    let title = plant ? "Potted \(name) made of colored points" : "Colombian flag made of colored points"
    var svg = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 \(width) \(height)\" role=\"img\" aria-labelledby=\"title desc\">\n"
    svg += "<title id=\"title\">\(title)</title>\n<desc id=\"desc\">Coarse colored circles preserving the supplied illustration's silhouette and main color regions.</desc>\n<g stroke=\"none\">\n"
    for point in points {
        let color = String(format: "#%02x%02x%02x", Int(point[2]), Int(point[3]), Int(point[4]))
        svg += "<circle cx=\"\(point[0])\" cy=\"\(point[1])\" r=\"\(radius)\" fill=\"\(color)\"/>\n"
    }
    svg += "</g>\n</svg>\n"
    try svg.write(toFile: stem + "-points.svg", atomically: true, encoding: .utf8)
    print("\(name): \(points.count) points, \(width) × \(height), target pitch \(targetPitch) CSS px")
}
