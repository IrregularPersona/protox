// the website this client hosts. change these two values to point the client at any site.
pub const TARGET_URL: &str = "https://alphatest.protox.io";
// bare host (no scheme, no trailing slash). used for resource swapper URL filters.
pub const TARGET_HOST: &str = "protox.io";

// native window class names of this client (main window + popups)
pub const WINDOW_CLASS: &str = "glorp_webview";
pub const WINDOW_CLASS_SUB: &str = "glorp_webview_subwindow";

pub const INSTANCE_MUTEX: &str = "Global\\7e0f405e-fe65-493a-acf0-9719b85697cd";

pub const EXAMPLE_CSS_OVERRIDE: &str = r#"/*
  Client CSS override.

  Everything in this file is appended to the game's stylesheet
  (assets/style-XXXXX.css) after it loads, so your rules win on equal
  specificity — no !important needed for ties.

  Requires a page refresh to apply.
*/

/* example:
#some-element {
    display: none !important;
}
*/"#;

pub const DEFAULT_BLOCKLIST: &str = r#"[
	"*://*.pollfish.com/*",
	"*://*.paypalobjects.com/*",
	"*://c.amazon-adsystem.com/*",
  "*://config.aps.amazon-adsystem.com/*",
  "*://securepubads.g.doubleclick.net/*",
  "*://cookiepro.com/*",
  "*://*.cookiepro.com/*",
  "*://cdn.ravenjs.com/*",
  "*://*.poll.fish/*",
  "*://*.paypal.com/*",
  "*://*.twitter.com/*",
  "*://*.youtube.com/*",
  "*://*.doubleclick.net/*",
  "*://unpkg.com/web3*",
  "*://storage.googleapis.com/pollfish_production/*",
  "*://*.googletagmanager.com/*",
  "*://apis.google.com/js/platform.js",
  "*://imasdk.googleapis.com/*",
  "*://*.googlesyndication.com/*",
  "*://www.google-analytics.com/*"
]"#;

// most are expired, but theyre all in here to cover multiple versions
pub const DEFAULT_FLAGS: &str = r#"[
  "--disable-features=PerformanceInterventionUI,site-isolation-trial-opt-out,slow-dc-timer-interrupts-win,NativeNotifications,webxr-runtime,enable-resource-loading-hint,MediaRouter,msWebOOUI,msPdfOOUI,msSmartScreenProtection,TextureLayerSkipWaitForActivation,CalculateNativeWinOcclusion,HappinessTrackingSurveysForDesktopDemo",
  "--ui-disable-partial-swap",
  "--disable-gpu-sandbox",
  "--ignore-gpu-blocklist",
  "--enable-gpu-rasterization",
  "--enable-webgl-draft-extensions",
  "--enable-zero-copy",
  "--enable-waitable-swap-chain",
  "--enable-unsafe-webgpu",
  "--disable-2d-canvas-clip-aa",
  "--disable-composited-antialiasing",
  "--disable-delegated-compositing",
  "--disable-software-rasterizer",
  "--disable-mipmap-generation",
  "--enable-native-gpu-memory-buffers",
  "--disable-gpu-driver-bug-workarounds",
  "--disable-gpu-watchdog",
  "--enable-features=SharedArrayBuffer,BlinkCompositorUseDisplayThreadPriority,GpuUseDisplayThreadPriority,BrowserUseDisplayThreadPriority,JavaScriptExperimentalSharedMemory,WebAssemblyBaseline,WebAssemblyTiering,WebAssemblyMemory64,WebAssemblyLazyCompilation,V8VmFuture",
  "--enable-accelerated-2d-canvas",
  "--disable-background-timer-throttling",
  "--disable-renderer-backgrounding",
  "--disable-best-effort-tasks",
  "--enable-threaded-compositing",
  "--raise-timer-frequency",
  "--wm-window-animations-disabled",
  "--enable-webassembly-threads",
  "--disable-low-end-device-mode",
  "--enable-future-v8-vm-features",
  "--enable-quic",
  "--quic-max-packet-length=1460",
  "--no-proxy-server",
  "--no-pings",
  "--dns-over-https=off",
  "--disable-logging",
  "--disable-metrics-repo",
  "--disable-metrics",
  "--disable-hang-monitor",
  "--disable-breakpad",
  "--disable-crash-reporter",
  "--disable-crashpad-forwarding",
  "--disable-oopr-debug-crash-dump",
  "--disable-in-process-stack-traces",
  "--disable-adpf",
  "--disable-bundled-ppapi-flash",
  "--disable-component-update",
  "--disable-nacl",
  "--disable-pnacl-crash-throttling",
  "--disable-threaded-scrolling",
  "--autoplay-policy=no-user-gesture-required",
  "--overscroll-history-navigation=0",
  "--pull-to-refresh=0"
]"#;
