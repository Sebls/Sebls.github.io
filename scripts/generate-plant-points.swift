// Compatibility entry point; shared sampler owns density and silhouette handling.
import Foundation
let task = Process()
task.executableURL = URL(fileURLWithPath: "/usr/bin/swift")
task.arguments = ["scripts/generate-decoration-points.swift", "plants"]
try task.run()
task.waitUntilExit()
exit(task.terminationStatus)
