// 可可桌宠 v0.1 — 交互层（气泡+菜单+面板）
import { bus } from './event-bus.js';

const menu = document.getElementById('context-menu');
const bubble = document.getElementById('bubble');
const bubbleText = document.getElementById('bubble-text');
const affectionLabel = document.getElementById('affection-label');
const affectionStage = document.getElementById('affection-stage');

let bubbleTimer = null;

// 气泡
bus.on('bubble', e => {
    bubbleText.textContent = e.detail || e;
    bubble.classList.remove('hidden');
    clearTimeout(bubbleTimer);
    bubbleTimer = setTimeout(() => bubble.classList.add('hidden'), 3000);
});

// 好感度更新
bus.on('affection:updated', e => {
    if (affectionLabel) affectionLabel.textContent = e.detail.value + '/1000';
    if (affectionStage) affectionStage.textContent = e.detail.tier || '';
});

// 右键菜单
bus.on('menu:toggle', e => {
    menu.classList.toggle('hidden');
    if (!menu.classList.contains('hidden')) {
        menu.style.left = Math.min(e.detail.x, window.innerWidth - 200) + 'px';
        menu.style.top = e.detail.y + 'px';
        document.getElementById('menu-affection').textContent = '❤️ ' + e.detail.affection;
    }
});

// 菜单项
document.querySelectorAll('.menu-item').forEach(item => {
    item.addEventListener('click', () => {
        const action = item.dataset.action;
        menu.classList.add('hidden');
        if (action === 'feed') showPanel('feed-panel');
        else if (action === 'stats') showPanel('stats-panel');
        else if (action === 'exit') window.__TAURI__?.window?.appWindow?.close();
    });
});

// 面板控制
function showPanel(id) {
    document.querySelectorAll('.panel').forEach(p => p.classList.add('hidden'));
    document.getElementById(id).classList.remove('hidden');
}
document.querySelectorAll('.panel-close').forEach(el => {
    el.addEventListener('click', () => el.closest('.panel').classList.add('hidden'));
});

// 喂食按钮
document.querySelector('[data-food="fish"]')?.addEventListener('click', () => bus.emit('feed:fish'));
document.querySelector('[data-food="can"]')?.addEventListener('click', () => bus.emit('feed:can'));

// 点击外部关闭
document.addEventListener('click', e => {
    if (!e.target.closest('#context-menu') && !e.target.closest('canvas')) menu.classList.add('hidden');
    if (!e.target.closest('.panel') && !e.target.closest('.menu-item[data-action="feed"],.menu-item[data-action="stats"]'))
        document.querySelectorAll('.panel').forEach(p => p.classList.add('hidden'));
});
