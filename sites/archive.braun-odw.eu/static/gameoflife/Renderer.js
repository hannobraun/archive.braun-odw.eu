(function(){function a(a){this._images=a,this.render=this.render.bind(this)}a.prototype.render=function(a,b,c,d){var e=[];for(var f=0;f<d.length;f++){var g=d[f].zIndex;e[g]==undefined&&(e[g]=[]),e[g].push(f)}a.clear(),a.saveState();var h=a.width/a.viewSize.x,i=a.height/a.viewSize.y;a.scale(h,i);var j=this;e.reverse().forEach(function(e){e.forEach(function(e){a.saveState();var f=b[e],g=d[e],h=f.x-a.position.x,i=f.y-a.position.y;a.translate(h,i),a.rotate(c[e]),a.scale(g.scale.x,g.scale.y),a.translate(g.offset.x,g.offset.y);var k=j._images[g.image];a.drawImage(k,0,0),a.restoreState()})}),a.restoreState()},typeof window=="undefined"?module.exports=a:register("spell/common/rendering/Renderer",a)})()