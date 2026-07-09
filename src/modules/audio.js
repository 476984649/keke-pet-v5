// 可可桌宠 v0.0.2 — Web Audio API 音效
import { bus } from './event-bus.js';
const ctx = new (window.AudioContext || window.webkitAudioContext)();
function p(fn) { try { fn(); } catch (e) {} }

bus.on('sfx:meow', () => p(() => {
    const o = ctx.createOscillator(), g = ctx.createGain(); o.connect(g); g.connect(ctx.destination);
    o.type = 'sine'; const t = ctx.currentTime;
    o.frequency.setValueAtTime(700, t); o.frequency.exponentialRampToValueAtTime(400, t + 0.12);
    o.frequency.exponentialRampToValueAtTime(900, t + 0.15); o.frequency.exponentialRampToValueAtTime(300, t + 0.3);
    g.gain.setValueAtTime(0.15, t); g.gain.exponentialRampToValueAtTime(0.01, t + 0.35);
    o.start(t); o.stop(t + 0.35);
}));

bus.on('sfx:whine', () => p(() => {
    const o = ctx.createOscillator(), g = ctx.createGain(); o.connect(g); g.connect(ctx.destination);
    o.type = 'sawtooth'; const t = ctx.currentTime;
    o.frequency.setValueAtTime(500, t); o.frequency.exponentialRampToValueAtTime(200, t + 0.3);
    g.gain.setValueAtTime(0.1, t); g.gain.exponentialRampToValueAtTime(0.01, t + 0.35);
    o.start(t); o.stop(t + 0.35);
}));

bus.on('sfx:sleepy', () => p(() => {
    const o = ctx.createOscillator(), g = ctx.createGain(); o.connect(g); g.connect(ctx.destination);
    o.type = 'sine'; const t = ctx.currentTime;
    o.frequency.setValueAtTime(600, t); o.frequency.exponentialRampToValueAtTime(150, t + 0.6);
    g.gain.setValueAtTime(0.1, t); g.gain.exponentialRampToValueAtTime(0.01, t + 0.7);
    o.start(t); o.stop(t + 0.7);
}));

bus.on('sfx:purr', () => p(() => {
    const o = ctx.createOscillator(), g = ctx.createGain(); o.connect(g); g.connect(ctx.destination);
    o.type = 'sine'; const t = ctx.currentTime;
    o.frequency.setValueAtTime(30, t);
    g.gain.setValueAtTime(0.08, t); g.gain.exponentialRampToValueAtTime(0.01, t + 1);
    o.start(t); o.stop(t + 1);
}));

bus.on('sfx:footstep', () => p(() => {
    const s = ctx.sampleRate * 0.05, buf = ctx.createBuffer(1, s, ctx.sampleRate), d = buf.getChannelData(0);
    for (let i = 0; i < s; i++) d[i] = (Math.random() * 2 - 1) * 0.06;
    const src = ctx.createBufferSource(); src.buffer = buf;
    const g = ctx.createGain(); g.gain.setValueAtTime(0.8, ctx.currentTime); g.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.05);
    src.connect(g); g.connect(ctx.destination); src.start();
}));
