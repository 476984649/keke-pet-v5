// 可可桌宠 — 2D骨骼动画引擎（透明背景处理）
const PARTS = {
    head:   { sx:25, sy:5,  sw:100, sh:90,  ax:75, ay:50 },
    body:   { sx:25, sy:65, sw:100, sh:70,  ax:75, ay:100 },
    tail:   { sx:100,sy:100,sw:50, sh:45,  ax:125,ay:120 },
    earL:   { sx:25, sy:0,  sw:30, sh:40,  ax:40, ay:15 },
    earR:   { sx:95, sy:0,  sw:30, sh:40,  ax:110,ay:15 },
    pawLF:  { sx:30, sy:125,sw:25, sh:40,  ax:42, ay:145 },
    pawRF:  { sx:95, sy:125,sw:25, sh:40,  ax:108,ay:145 },
};

let transparentImg = null;

// 抠图：白色→透明
function makeTransparent(img, w, h) {
    const c = document.createElement('canvas'); c.width = w; c.height = h;
    const cx = c.getContext('2d'); cx.drawImage(img, 0, 0);
    const d = cx.getImageData(0, 0, w, h);
    for (let i = 0; i < d.data.length; i += 4) {
        if (d.data[i] > 250 && d.data[i+1] > 248 && d.data[i+2] > 245)
            d.data[i+3] = 0;
    }
    cx.putImageData(d, 0, 0);
    return c;
}

function getTransparentImg(img) {
    if (!transparentImg && img.complete && img.naturalWidth > 0)
        transparentImg = makeTransparent(img, img.naturalWidth, img.naturalHeight);
    return transparentImg || img;
}

function bonePose(state, affection, t) {
    const p = { headTilt:0, earAngle:0, tailWag:0, bodyBob:0, pawShift:0, eyeScale:1, mouthOpen:0 };
    switch(state) {
    case 'idle': p.bodyBob = Math.sin(t*1.5)*1.5; p.tailWag = Math.sin(t*2)*3; break;
    case 'walking': p.bodyBob = Math.abs(Math.sin(t*6))*3; p.pawShift = Math.sin(t*6)*4; p.tailWag = Math.sin(t*4)*5; break;
    case 'sleeping': p.headTilt=10; p.bodyBob=Math.sin(t*0.5)*1; p.tailWag=Math.sin(t*0.3)*2; p.eyeScale=0; break;
    case 'petting': p.headTilt=8; p.earAngle=-8; p.eyeScale=0.2; p.tailWag=Math.sin(t*8)*8; break;
    case 'eating': p.headTilt=12; p.mouthOpen=0.5; p.earAngle=5; p.tailWag=Math.sin(t*4)*5; break;
    case 'held': p.pawShift=-5; p.earAngle=-12; break;
    case 'organizing': p.headTilt=-3; p.pawShift=Math.sin(t*8)*3; p.tailWag=Math.sin(t*6)*4; break;
    case 'overtime': p.headTilt=12; p.eyeScale=0.5; p.bodyBob=Math.sin(t*0.7)*2; p.tailWag=Math.sin(t*0.5)*1; break;
    case 'gift': p.tailWag=Math.sin(t*10)*10; p.earAngle=8; break;
    }
    p.tailWag *= 1 + affection/500;
    return p;
}

function drawPart(ctx, img, name, pose) {
    const part = PARTS[name]; if (!part) return;
    const src = getTransparentImg(img);
    ctx.save(); ctx.translate(part.ax, part.ay);
    let rot=0, dx=0, dy=0;
    switch(name) {
    case 'head': rot=pose.headTilt*0.03; dy=pose.bodyBob; break;
    case 'earL': rot=-pose.earAngle*0.05; break;
    case 'earR': rot=pose.earAngle*0.05; break;
    case 'tail': rot=pose.tailWag*0.04; break;
    case 'body': dy=pose.bodyBob; break;
    case 'pawLF': dy=pose.pawShift; break;
    case 'pawRF': dy=-pose.pawShift; break;
    }
    ctx.rotate(rot);
    ctx.drawImage(src, part.sx, part.sy, part.sw, part.sh, -part.sw/2+dx, -part.sh/2+dy, part.sw, part.sh);
    ctx.restore();
}

export { PARTS, bonePose, drawPart, getTransparentImg };
