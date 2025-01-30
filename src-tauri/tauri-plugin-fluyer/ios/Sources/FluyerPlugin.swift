import SwiftRs
import Tauri
import UIKit
import WebKit

// class PingArgs: Decodable {
//   let value: String?
// }

class FluyerPlugin: Plugin {
//  @objc public func ping(_ invoke: Invoke) throws {
//    let args = try invoke.parseArgs(PingArgs.self)
//    invoke.resolve(["value": args.value ?? ""])
//  }
}

@_cdecl("init_plugin_fluyer")
func initPlugin() -> Plugin {
  return FluyerPlugin()
}
