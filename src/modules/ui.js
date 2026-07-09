// 可可桌宠 v0.5 — 交互层
import { bus } from './event-bus.js';

const menu = document.getElementById('context-menu');
const bubble = document.getElementById('bubble');
const bubbleText = document.getElementById('bubble-text');
const affectionLabel = document.getElementById('affection-label');
const affectionStage = document.getElementById('affection-stage');

let bubbleTimer = null;

bus.on('bubble', e => {
    bubbleText.textContent = e.detail || e;
    bubble.classList.remove('hidden');
    clearTimeout(bubbleTimer);
    bubbleTimer = setTimeout(() => bubble.classList.add('hidden'), 3000);
});

bus.on('affection:updated', e => {
    if (affectionLabel) affectionLabel.textContent = e.detail.value + '/1000';
    if (affectionStage) affectionStage.textContent = e.detail.tier || '';
});

// 右键菜单 — 窗口内定位
bus.on('menu:toggle', e => {
    menu.classList.toggle('hidden');
    if (!menu.classList.contains('hidden')) {
        menu.style.left = Math.min(Math.max(e.detail.x - 10, 2), 140) + 'px';
        menu.style.top = Math.min(e.detail.y - 10, 80) + 'px';
        document.getElementById('menu-affection').textContent = '❤️ ' + e.detail.affection;
    }
});

// 菜单项点击 → 关闭菜单
document.querySelectorAll('.menu-item').forEach(item => {
    item.addEventListener('click', () => {
        const action = item.dataset.action;
        menu.classList.add('hidden');
        if (action === 'feed') { document.getElementById('feed-panel').classList.remove('hidden'); }
        else if (action === 'stats') { document.getElementById('stats-panel').classList.remove('hidden'); }
        else if (action === 'exit') window.__TAURI__?.window?.appWindow?.close();
    });
});

// 面板关闭
document.querySelectorAll('.panel-close').forEach(el => {
    el.addEventListener('click', () => el.closest('.panel').classList.add('hidden'));
});

// 喂食按钮
document.querySelector('[data-food="fish"]')?.addEventListener('click', () => {
    bus.emit('feed:fish');
    document.getElementById('feed-panel').classList.add('hidden');
});
document.querySelector('[data-food="can"]')?.addEventListener('click', () => {
    bus.emit('feed:can');
    document.getElementById('feed-panel').classList.add('hidden');
});

// 点击外部关闭
document.addEventListener('click', e => {
    if (!e.target.closest('#context-menu') && !e.target.closest('canvas')) menu.classList.add('hidden');
    if (!e.target.closest('.panel') && !e.target.closest('.menu-item'))
        document.querySelectorAll('.panel').forEach(p => p.classList.add('hidden'));
});
