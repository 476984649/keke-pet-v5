// 可可桌宠 v0.5 — Live2D 入口 + Canvas 降级
let useLive2D = false;

async function initRenderer() {
    try {
        // Phase 2: 尝试加载 Live2D 模型
        // Live2D Cubism 5 SDK 路径预留
        // await loadLive2DModel('/models/keke/keke.model3.json');
        // useLive2D = true;
        // console.log('Live2D loaded');
        throw new Error('Live2D model not found');
    } catch (e) {
        console.warn('Live2D 加载失败，降级至 Canvas 2D:', e.message);
        useLive2D = false;
        // Canvas fallback already loaded via renderer.js
    }
}

initRenderer();
export { useLive2D };
