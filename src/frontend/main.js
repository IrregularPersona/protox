import "./utils.js";

window.closeClient = () => window.chrome.webview.postMessage("close");

window.glorp = new Promise((resolve) => {
	function handler(event) {
		if (event?.data?.settings || event?.data?.version) {
			window.chrome.webview.removeEventListener("message", handler);
			resolve(event.data);
		}
	}

	window.chrome.webview.addEventListener("message", handler);
	window.chrome.webview.postMessage("get-info");
}).then((data) => (window.glorp = data));

document.addEventListener(
	"DOMContentLoaded",
	async () => {
		// limit the rAF loop if configured
		import("./modules/gameFpsLimit.js");

		// wait for window.glorp to resolve
		if (window.glorp instanceof Promise) await window.glorp;

		import("./notifications.js");

		hook(HTMLCanvasElement, "addEventListener", (args) => {
			const [type, listener] = args;
			if (type === "wheel") window.glorp.handleMouseWheel = (deltaY) => listener(new WheelEvent("wheel", { deltaY }));
		});

		hook(HTMLCanvasElement, "requestPointerLock", function (args, original) {
			window.chrome.webview.postMessage("drag, false");
			window.chrome.webview.postMessage("throttle, game");

			return original.call(this, { ...args[0], unadjustedMovement: window.glorp?.settings?.data?.rawInput });
		});

		document.addEventListener("pointerlockchange", () => {
			if (!document.pointerLockElement) {
				window.chrome.webview.postMessage("drag, true");
				window.chrome.webview.postMessage("throttle, menu");
			} else {
				window.chrome.webview.postMessage("throttle, game");
			}
		});
	},
	{ once: true },
);
