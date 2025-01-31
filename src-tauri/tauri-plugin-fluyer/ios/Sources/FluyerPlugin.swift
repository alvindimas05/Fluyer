import SwiftRs
import Tauri
import UIKit
import WebKit

class FluyerPlugin: Plugin {
    @objc open override func load(webview: WKWebView) {
        guard let rootViewController = UIApplication.shared.windows.first?.rootViewController else { return }
        
        rootViewController.edgesForExtendedLayout = .all
        rootViewController.extendedLayoutIncludesOpaqueBars = true
        
        webview.backgroundColor = .clear
        webview.scrollView.backgroundColor = .clear
        webview.scrollView.contentInsetAdjustmentBehavior = .never
        
        webview.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        webview.frame = rootViewController.view.bounds
    }
    
    @objc public func getStatusBarHeight(_ invoke: Invoke){
        invoke.resolve(["value": UIApplication.shared.windows.first?.windowScene?.statusBarManager?.statusBarFrame.height ?? 0])
    }
    
    @objc public func getNavigationBarHeight(_ invoke: Invoke){
        invoke.resolve(["value": UIApplication.shared.windows.first?.safeAreaInsets.bottom ?? 0])
    }
}

@_cdecl("init_plugin_fluyer")
func initPlugin() -> Plugin {
  return FluyerPlugin()
}
