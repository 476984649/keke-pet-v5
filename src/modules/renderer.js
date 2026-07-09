// 可可桌宠 v0.1 — 完整渲染引擎
import { bus } from './event-bus.js';

const W=150,H=200,canvas=document.getElementById('keke-canvas'),ctx=canvas.getContext('2d');
canvas.width=W;canvas.height=H;

const img=new Image();img.src='/images/keke_base.png';

// ===== 状态 =====
let state='idle',direction=1,affection=0,idleTime=0,blinkTimer=0,isBlinking=0;
let x=W/2,y=H,dragOffX=0,dragOffY=0,targetX=W/2,targetY=H,isMoving=0,petTimer=0,eatTimer=0;
let startTime=Date.now(),isSleeping=0;
let foodCooldowns=[0,0,0];//fish,can
let keyboardActivity=0;

function sfx(n){bus.emit('sfx:'+n)}

// ===== 好感度分阶 =====
function tier(){return affection>=1000?'羁绊 💖':affection>=600?'家人 💕':affection>=300?'亲密 ❤️':affection>=100?'熟悉 ♥️':'陌生'}

// ===== 主循环 =====
function draw(){
    requestAnimationFrame(draw);ctx.clearRect(0,0,W,H);
    if(!img.complete||img.naturalWidth===0)return;idleTime++;
    const t=(Date.now()-startTime)/1000;
    const breathY=Math.sin(t*Math.PI/2)*3,breathScale=1+Math.sin(t*Math.PI/2)*0.02;
    blinkTimer++;if(!isBlinking&&blinkTimer>120+Math.random()*80){isBlinking=1;blinkTimer=0}if(isBlinking&&blinkTimer>8)isBlinking=0;

    // 冷却更新
    for(let i=0;i<2;i++)if(foodCooldowns[i]>0)foodCooldowns[i]--;

    // ===== 状态机 =====
    if(state==='sleeping'){drawKeke(breathY,0.9,0);if(Math.random()<0.002)sfx('sleepy');return}
    if(state==='petting'){petTimer++;drawKeke(breathY-2,1.03,1);if(petTimer>30){state='idle';idleTime=0}return}
    if(state==='held'){drawKeke(breathY,0.85,0);return}
    if(state==='eating'){eatTimer++;drawKeke(breathY-3,1.05,1);if(eatTimer>40){state='idle';idleTime=0}return}

    // 专属行为触发
    if(state==='idle'){
        const h=new Date().getHours();
        // 陪加班
        if(h>=22||h<=4){state='overtime';bus.emit('bubble','这么晚还在呀...');setTimeout(()=>{if(state==='overtime'){state='idle';idleTime=0}},3000)}
        // 叼礼物(好感度里程碑)
        else if(affection>=100&&affection<101&&Math.random()<0.01){state='gift';bus.emit('bubble','🎁 给你的！');setTimeout(()=>{state='idle';idleTime=0},2500)}
        else if(affection>=300&&affection<301&&Math.random()<0.01){state='gift';bus.emit('bubble','🎁 又叼来礼物啦~');setTimeout(()=>{state='idle';idleTime=0},2500)}
        // 睡觉(5min≈18000帧)
        else if(idleTime>18000){state='sleeping';isSleeping=1;bus.emit('bubble','zzz...')}
    }
    if(state==='sleeping'&&!isSleeping){isSleeping=1;bus.emit('bubble','zzz...')}

    // 走路/随机行为
    if(state==='idle'&&idleTime>200&&Math.random()<0.008){
        const r=Math.random();
        if(r<0.25){state='walking';sfx('footstep')}
        else if(r<0.35&&affection>=100){state='organizing';bus.emit('bubble','📋 整理一下~');setTimeout(()=>{state='idle';idleTime=0},2000)}
        else if(r<0.45&&affection>=200){state='supervisor';bus.emit('bubble','👀 认真工作~');setTimeout(()=>{state='idle';idleTime=0},3000)}
        else if(r<0.50&&affection>=300){state='slacker';bus.emit('bubble','⚠️ 别摸鱼啦！');setTimeout(()=>{state='idle';idleTime=0},2500)}
        else state='idle';
    }
    if(state==='walking'||(state==='idle'&&idleTime>100&&Math.random()<0.01)){
        if(state==='idle'){state='walking';sfx('footstep')}
        if(isMoving){const dx=targetX-x,dy=targetY-y,dist=Math.sqrt(dx*dx+dy*dy);if(dist>3){x+=dx/dist*1.5;y+=dy/dist*1.5;direction=dx>0?1:-1;if(Math.random()<0.03)sfx('footstep')}else{state='idle';idleTime=0;isMoving=0}}
        else{targetX=20+Math.random()*(W-40);targetY=20+Math.random()*(H-40);isMoving=1}
        drawKeke(breathY,breathScale,0);return
    }
    drawKeke(breathY,breathScale,isBlinking)
}

function drawKeke(breathY,scale,squint){
    const dx=x-75+(state==='sleeping'?5:0),dy=y-200+breathY+(state==='sleeping'?10:0);
    const dw=150*scale,dh=200*scale,ox=(dw-150)/2,oy=dh-200;
    ctx.save();ctx.translate(dx+75,dy+100);if(direction===-1)ctx.scale(-1,1);
    if(state==='sleeping'){ctx.scale(0.85,0.85);ctx.rotate(-0.3)}
    ctx.drawImage(img,-75-ox,-100-oy,dw,dh);
    if(squint||isBlinking){ctx.fillStyle='rgba(255,245,238,0.85)';ctx.fillRect(-20,-30,18,5);ctx.fillRect(4,-30,18,5)}
    ctx.restore();
}

// ===== 鼠标事件 =====
let clickTimer=null;
canvas.addEventListener('mousedown',e=>{
    if(state==='sleeping'){state='idle';isSleeping=0;sfx('meow');return}
    dragOffX=e.clientX-x;dragOffY=e.clientY-y;state='held';sfx('whine');
});
canvas.addEventListener('mousemove',e=>{if(state==='held'){x=e.clientX-dragOffX;y=e.clientY-dragOffY}});
document.addEventListener('mouseup',()=>{if(state==='held'){state='idle';idleTime=0}});
canvas.addEventListener('click',()=>{
    if(state==='held')return;
    if(clickTimer){clearTimeout(clickTimer);clickTimer=null;return}
    clickTimer=setTimeout(()=>{clickTimer=null;if(state==='sleeping'){state='idle';isSleeping=0;return}state='petting';petTimer=0;affection=Math.min(1000,affection+1);sfx('meow');bus.emit('affection:updated',{value:affection,tier:tier()})},250);
});
canvas.addEventListener('contextmenu',e=>{e.preventDefault();bus.emit('menu:toggle',{x:e.clientX,y:e.clientY,affection,tier:tier()})});

// ===== 喂食 =====
bus.on('feed:fish',()=>{
    if(foodCooldowns[0]>0){bus.emit('bubble','🐟 冷却中...');return}
    state='eating';eatTimer=0;foodCooldowns[0]=18000;//5min
    affection=Math.min(1000,affection+5);sfx('meow');
    bus.emit('bubble','🐟 小鱼干！+5❤️');bus.emit('affection:updated',{value:affection,tier:tier()});
});
bus.on('feed:can',()=>{
    if(foodCooldowns[1]>0){bus.emit('bubble','🥫 冷却中...');return}
    state='eating';eatTimer=0;foodCooldowns[1]=108000;//30min
    affection=Math.min(1000,affection+15);sfx('meow');
    bus.emit('bubble','🥫 猫罐头！+15❤️');bus.emit('affection:updated',{value:affection,tier:tier()});
});

// ===== 传播钩子 =====
document.addEventListener('keydown',()=>{
    keyboardActivity++;
    if(keyboardActivity>300&&Math.random()<0.002&&state==='idle'){keyboardActivity=0;state='walking';targetX=W*0.5;targetY=H*0.7;isMoving=1;bus.emit('bubble','⌨️ 踩踩踩~');setTimeout(()=>{state='idle';idleTime=0},2000)}
});
setInterval(()=>{if(state==='idle'&&Math.random()<0.003&&affection>30){state='walking';targetX=W*0.15;targetY=H*0.3;isMoving=1;bus.emit('bubble','📱 推推~');setTimeout(()=>{state='idle';idleTime=0},1500)}},30000);

img.onload=()=>{draw();bus.emit('keke:ready')};
img.onerror=()=>{ctx.fillStyle='#FFF5EE';ctx.fillRect(0,0,W,H);ctx.fillStyle='#7EB5E0';ctx.beginPath();ctx.arc(75,70,40,0,Math.PI*2);ctx.fill();ctx.fillStyle='#FFF';ctx.font='14px sans-serif';ctx.fillText('可可',56,160);draw()};
