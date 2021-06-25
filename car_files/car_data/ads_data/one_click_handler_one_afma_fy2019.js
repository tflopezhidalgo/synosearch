(function(){/* 
 
 Copyright The Closure Library Authors. 
 SPDX-License-Identifier: Apache-2.0 
*/ 
'use strict';var n=this||self;function p(a,b){return a.g?a.j.slice(0,a.g.index)+b+a.j.slice(a.g.index):a.j+b}function q(a,b){return a.i&&a.h||a.l?1==b?a.i?a.h:p(a,"&dct=1"):2==b?p(a,"&ri=2"):p(a,"&ri=16"):a.j}var aa=class{constructor({url:a}){this.j=a;const b=/[?&]dsh=1(&|$)/.test(a);this.i=!b&&/[?&]ae=1(&|$)/.test(a);this.l=!b&&/[?&]ae=2(&|$)/.test(a);if((this.g=/[?&]adurl=([^&]*)/.exec(a))&&this.g[1]){let c;try{c=decodeURIComponent(this.g[1])}catch(d){c=null}this.h=c}}};function r(a){return-1!=ba.indexOf(a)};var t=class{constructor(a,b){this.g=b===ca?a:""}};t.prototype.i=!0;t.prototype.h=function(){return this.g.toString()};t.prototype.toString=function(){return this.g.toString()};function da(a){return a instanceof t&&a.constructor===t?a.g:"type_error:SafeUrl"}var ea=/^(?:(?:https?|mailto|ftp):|[^:/?#]*(?:[/?#]|$))/i,ca={};function fa(a,b){a:{const c=a.length,d="string"===typeof a?a.split(""):a;for(let e=0;e<c;e++)if(e in d&&b.call(void 0,d[e],e,a)){b=e;break a}b=-1}return 0>b?null:"string"===typeof a?a.charAt(b):a[b]};var ba;a:{var ha=n.navigator;if(ha){var ia=ha.userAgent;if(ia){ba=ia;break a}}ba=""};function ka(a){let b=!1,c;return function(){b||(c=a(),b=!0);return c}};function u(a,b){b instanceof t||b instanceof t||(b="object"==typeof b&&b.i?b.h():String(b),ea.test(b)||(b="about:invalid#zClosurez"),b=new t(b,ca));a.href=da(b)};function la(){return Math.floor(2147483648*Math.random()).toString(36)+Math.abs(Math.floor(2147483648*Math.random())^Date.now()).toString(36)};function ma(){return r("iPhone")&&!r("iPod")&&!r("iPad")};function oa(a){oa[" "](a);return a}oa[" "]=function(){};var pa=ma(),qa=r("iPad");var ra=ma()||r("iPod"),sa=r("iPad");var ta={},ua=null;var va="function"===typeof Uint8Array;function wa(a){return xa(a,b=>b,b=>new Uint8Array(b))}function ya(a,b,c){return"object"===typeof a?va&&!Array.isArray(a)&&a instanceof Uint8Array?c(a):xa(a,b,c):b(a)}function xa(a,b,c){if(Array.isArray(a)){var d=Array(a.length);for(var e=0;e<a.length;e++){var f=a[e];null!=f&&(d[e]=ya(f,b,c))}Array.isArray(a)&&a[za]&&v(d);return d}d={};for(e in a)f=a[e],null!=f&&(d[e]=ya(f,b,c));return d}const za=Symbol("IS_REPEATED_FIELD"); 
function v(a){Array.isArray(a)&&!Object.isFrozen(a)&&(a[za]=!0);return a};function Aa(a){if(a.i){if(a.j){var b=a.g;for(d in b)if(Object.prototype.hasOwnProperty.call(b,d)){var c=b[d].g;c&&c.m()}}}else{a.h.length=0;var d=y(a);d.sort();for(b=0;b<d.length;b++){let e=a.g[d[b]];(c=e.g)&&c.m();a.h.push([e.key,e.value])}a.i=!0}return a.h}function y(a){a=a.g;var b=[],c;for(c in a)Object.prototype.hasOwnProperty.call(a,c)&&b.push(c);return b}function Ba(a,b){return a.j?(b.g||(b.g=new a.j(b.value),a.isFrozen()&&null(b.g)),b.g):b.value} 
function Ca(a,b){var c=y(a);c.sort();for(var d=0;d<c.length;d++){let e=a.g[c[d]];b.call(void 0,Ba(a,e),e.key,a)}} 
class Da{constructor(a,b){this.h=a;this.j=b;this.g={};this.i=!0;if(0<this.h.length){for(a=0;a<this.h.length;a++){b=this.h[a];var c=b[0];this.g[c.toString()]=new Ha(c,b[1])}this.i=!0}}isFrozen(){return!1}m(){return Aa(this)}I(){return Aa(this)}entries(){var a=[],b=y(this);b.sort();for(var c=0;c<b.length;c++){let d=this.g[b[c]];a.push([d.key,Ba(this,d)])}return new Ia(a)}keys(){var a=[],b=y(this);b.sort();for(var c=0;c<b.length;c++)a.push(this.g[b[c]].key);return new Ia(a)}values(){var a=[],b=y(this); 
b.sort();for(var c=0;c<b.length;c++)a.push(Ba(this,this.g[b[c]]));return new Ia(a)}set(a,b){var c=new Ha(a);this.j?(c.g=b,c.value=b.I()):c.value=b;this.g[a.toString()]=c;this.i=!1;return this}get(a){if(a=this.g[a.toString()])return Ba(this,a)}has(a){return a.toString()in this.g}[Symbol.iterator](){return this.entries()}}class Ha{constructor(a,b){this.key=a;this.value=b;this.g=void 0}} 
class Ia{constructor(a){this.h=0;this.g=a}next(){return this.h<this.g.length?{done:!1,value:this.g[this.h++]}:{done:!0,value:void 0}}[Symbol.iterator](){return this}};function z(){}let A;function B(a,b,c){a.g=null;A&&(b||(b=A),A=null);var d=a.constructor.O;b||(b=d?[d]:[]);a.j=d?0:-1;a.h=b;a:{if(b=a.h.length)if(--b,d=a.h[b],!(null===d||"object"!=typeof d||Array.isArray(d)||va&&d instanceof Uint8Array)){a.l=b-a.j;a.i=d;break a}a.l=Number.MAX_VALUE}a.o={};if(c)for(b=0;b<c.length;b++)if(d=c[b],d<a.l){d+=a.j;var e=a.h[d];e?v(e):a.h[d]=C}else Ja(a),(e=a.i[d])?v(e):a.i[d]=C}const C=Object.freeze(v([]));function Ja(a){let b=a.l+a.j;a.h[b]||(a.i=a.h[b]={})} 
function F(a,b){if(b<a.l){b+=a.j;var c=a.h[b];return c!==C?c:a.h[b]=v([])}if(a.i)return c=a.i[b],c!==C?c:a.i[b]=v([])}function G(a,b,c){a=F(a,b);return null==a?c:a}function H(a,b){return G(a,b,"")}function K(a,b){a=F(a,b);a=null==a?a:!!a;return null==a?!1:a}function L(a,b,c){a.g||(a.g={});if(b in a.g)return a.g[b];let d=F(a,b);d||(d=v([]),M(a,b,d));c=new Da(d,c);return a.g[b]=c}function M(a,b,c){b<a.l?a.h[b+a.j]=c:(Ja(a),a.i[b]=c);return a} 
function N(a,b,c){a.g||(a.g={});if(!a.g[c]){let d=F(a,c);d&&(a.g[c]=new b(d))}return a.g[c]}function Ka(a){var b=La;a.g||(a.g={});if(!a.g[7]){let c=F(a,7),d=[];for(let e=0;e<c.length;e++)d[e]=new b(c[e]);a.g[7]=d}b=a.g[7];b==C&&(b=a.g[7]=[]);return b}function Ma(a){if(a.g)for(var b in a.g){var c=a.g[b];if(Array.isArray(c))for(var d=0;d<c.length;d++)c[d]&&c[d].m();else c&&c.m()}}z.prototype.m=function(){Ma(this);return this.h};z.prototype.I=function(){Ma(this);return this.h}; 
function Na(a,b){switch(typeof b){case "number":return isNaN(b)||Infinity===b||-Infinity===b?String(b):b;case "object":if(va&&null!=b&&b instanceof Uint8Array){var c;void 0===c&&(c=0);if(!ua){ua={};a="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".split("");for(var d=["+/=","+/","-_=","-_.","-_"],e=0;5>e;e++){var f=a.concat(d[e].split(""));ta[e]=f;for(var g=0;g<f.length;g++){var k=f[g];void 0===ua[k]&&(ua[k]=g)}}}c=ta[c];a=Array(Math.floor(b.length/3));d=c[64]||"";for(e=f=0;f<b.length- 
2;f+=3){var l=b[f],h=b[f+1];k=b[f+2];g=c[l>>2];l=c[(l&3)<<4|h>>4];h=c[(h&15)<<2|k>>6];k=c[k&63];a[e++]=g+l+h+k}g=0;k=d;switch(b.length-f){case 2:g=b[f+1],k=c[(g&15)<<2]||d;case 1:b=b[f],a[e]=c[b>>2]+c[(b&3)<<4|g>>4]+k+d}return a.join("")}}return b}z.prototype.toString=function(){return this.m().toString()};function Oa(a){const b=wa(a.m());A=b;a=new a.constructor(b);A=null;return a};var Pa=class extends z{constructor(a){super();B(this,a,null)}};var Qa=class extends z{constructor(a){super();B(this,a,null)}};var Sa=class extends z{constructor(a){super();B(this,a,Ra)}},La=class extends z{constructor(a){super();B(this,a,null)}s(){return H(this,3)}H(a){M(this,5,a)}},Ta=class extends z{constructor(a){super();B(this,a,null)}s(){return H(this,1)}H(a){M(this,2,a)}},Ua=class extends z{constructor(a){super();B(this,a,null)}},Ra=[6,7];var Wa=class extends z{constructor(a){super();B(this,a,Va)}},Va=[17];var Xa=class extends z{constructor(a){super();B(this,a,null)}};var Ya={};var Za=class{};class $a extends Za{constructor(a){super();if(Ya!==Ya)throw Error("Bad secret");this.g=a}toString(){return this.g}}var ab=new $a("about:invalid#zTSz");function bb(a){if(a instanceof Za)if(a instanceof $a)a=a.g;else throw Error("wrong type");else a=da(a);return a};class cb{constructor(a,b){this.error=a;this.context=b.context;this.msg=b.message||"";this.id=b.id||"jserror";this.meta={}}};var db={capture:!0},eb={passive:!0},fb=ka(function(){let a=!1;try{const b=Object.defineProperty({},"passive",{get:function(){a=!0}});n.addEventListener("test",null,b)}catch(b){}return a});function gb(a){return a?a.passive&&fb()?a:a.capture||!1:!1}function O(a,b,c,d){a.addEventListener&&a.addEventListener(b,c,gb(d))}function ib(a,b,c){a.removeEventListener&&a.removeEventListener(b,c,gb(void 0))};var jb=/^(?:([^:/?#.]+):)?(?:\/\/(?:([^\\/?#]*)@)?([^\\/?#]*?)(?::([0-9]+))?(?=[\\/?#]|$))?([^?#]+)?(?:\?([^#]*))?(?:#([\s\S]*))?$/;function kb(a){var b=a.indexOf("#");0>b&&(b=a.length);var c=a.indexOf("?");if(0>c||c>b){c=b;var d=""}else d=a.substring(c+1,b);return[a.substr(0,c),d,a.substr(b)]}function lb(a,b){return b?a?a+"&"+b:b:a}function nb(a,b){if(!b)return a;a=kb(a);a[1]=lb(a[1],b);return a[0]+(a[1]?"?"+a[1]:"")+a[2]} 
function ob(a,b,c){if(Array.isArray(b))for(var d=0;d<b.length;d++)ob(a,String(b[d]),c);else null!=b&&c.push(a+(""===b?"":"="+encodeURIComponent(String(b))))}function pb(a){var b=[],c;for(c in a)ob(c,a[c],b);return b.join("&")}function qb(){var a=la();a=null!=a?"="+encodeURIComponent(String(a)):"";return nb("https://pagead2.googlesyndication.com/pagead/gen_204","zx"+a)} 
function rb(a,b){a=kb(a);var c=a[1],d=[];c&&c.split("&").forEach(function(e){var f=e.indexOf("=");b.hasOwnProperty(0<=f?e.substr(0,f):e)||d.push(e)});a[1]=lb(d.join("&"),pb(b));return a[0]+(a[1]?"?"+a[1]:"")+a[2]};function sb(a){try{var b;if(b=!!a&&null!=a.location.href)a:{try{oa(a.foo);b=!0;break a}catch(c){}b=!1}return b}catch(c){return!1}}function tb(a,b){if(a)for(const c in a)Object.prototype.hasOwnProperty.call(a,c)&&b.call(void 0,a[c],c,a)}let ub=[];const vb=()=>{const a=ub;ub=[];for(const b of a)try{b()}catch(c){}}; 
var wb=a=>{ub.push(a);1==ub.length&&(window.Promise?Promise.resolve().then(vb):window.setImmediate?setImmediate(vb):setTimeout(vb,0))},xb=a=>{var b=P;"complete"===b.readyState||"interactive"===b.readyState?wb(a):b.addEventListener("DOMContentLoaded",a)},yb=a=>{var b=window;"complete"===b.document.readyState?wb(a):b.addEventListener("load",a)};function Q(a,b,c=null){zb(a,b,c)}function zb(a,b,c){a.google_image_requests||(a.google_image_requests=[]);const d=a.document.createElement("img");if(c){const e=f=>{c&&c(f);ib(d,"load",e);ib(d,"error",e)};O(d,"load",e);O(d,"error",e)}d.src=b;a.google_image_requests.push(d)};let Ab=0;function Bb(a){return(a=Cb(a,document.currentScript))&&a.getAttribute("data-jc-version")||"unknown"}function Cb(a,b=null){return b&&b.getAttribute("data-jc")===String(a)?b:document.querySelector(`[${"data-jc"}="${a}"]`)} 
function Db(a){if(!(.01<Math.random())){a=`https://${"pagead2.googlesyndication.com"}/pagead/gen_204?id=jca&jc=${a}&version=${Bb(a)}&sample=${.01}`;var b=window,c;if(c=b.navigator)c=b.navigator.userAgent,c=/Chrome/.test(c)&&!/Edge/.test(c)?!0:!1;c&&b.navigator.sendBeacon?b.navigator.sendBeacon(a):Q(b,a)}};var P=document,R=window;class Eb{constructor(a){this.N=a}}function S(a){return new Eb(b=>b.substr(0,a.length+1).toLowerCase()===a+":")}const Fb=new Eb(a=>/^[^:]*([/?#]|$)/.test(a));var Gb=S("http"),Hb=S("https"),Ib=S("ftp"),Jb=S("mailto");const Kb=[S("data"),Gb,Hb,Jb,Ib,Fb];function Pb(a,b=Kb){for(let c=0;c<b.length;++c){const d=b[c];if(d instanceof Eb&&d.N(a))return new $a(a)}}function Qb(a,b=Kb){return Pb(a,b)||ab};const Rb=[Gb,Hb,Jb,Ib,Fb,S("market"),S("itms"),S("intent"),S("itms-appss")];function Sb(a,b){if(a instanceof t)return a;const c=Qb(a,Rb);c===ab&&b(a);return new t(bb(c),ca)}var Tb=a=>{var b=`${"http:"===R.location.protocol?"http:":"https:"}//${"pagead2.googlesyndication.com"}/pagead/gen_204`;return c=>{c=pb({id:"unsafeurl",ctx:a,url:c});c=nb(b,c);navigator.sendBeacon&&navigator.sendBeacon(c,"")}};var Ub=!!window.google_async_iframe_id;let T=Ub&&window.parent||window;var Vb=a=>{var b=P;try{return b.querySelectorAll("*["+a+"]")}catch(c){return[]}};const Wb=/^https?:\/\/(\w|-)+\.cdn\.ampproject\.(net|org)(\?|\/|$)/;var Xb=class{constructor(a,b){this.g=a;this.h=b}},Yb=class{constructor(a,b){this.url=a;this.G=!!b;this.depth=null}};function Zb(a,b){const c={};c[a]=b;return[c]}function $b(a,b,c,d,e){const f=[];tb(a,function(g,k){(g=ac(g,b,c,d,e))&&f.push(k+"="+g)});return f.join(b)} 
function ac(a,b,c,d,e){if(null==a)return"";b=b||"&";c=c||",$";"string"==typeof c&&(c=c.split(""));if(a instanceof Array){if(d=d||0,d<c.length){const f=[];for(let g=0;g<a.length;g++)f.push(ac(a[g],b,c,d+1,e));return f.join(c[d])}}else if("object"==typeof a)return e=e||0,2>e?encodeURIComponent($b(a,b,c,d,e+1)):"...";return encodeURIComponent(String(a))}function bc(a){let b=1;for(const c in a.h)b=c.length>b?c.length:b;return 3997-b-a.i.length-1} 
function cc(a,b,c){b=b+"//pagead2.googlesyndication.com"+c;let d=bc(a)-c.length;if(0>d)return"";a.g.sort(function(f,g){return f-g});c=null;let e="";for(let f=0;f<a.g.length;f++){const g=a.g[f],k=a.h[g];for(let l=0;l<k.length;l++){if(!d){c=null==c?g:c;break}let h=$b(k[l],a.i,",$");if(h){h=e+h;if(d>=h.length){d-=h.length;b+=h;e=a.i;break}c=null==c?g:c}}}a="";null!=c&&(a=e+"trn="+c);return b+a}class dc{constructor(){this.i="&";this.h={};this.j=0;this.g=[]}};function ec(a,b,c,d,e,f){if((d?a.g:Math.random())<(e||.01))try{let g;c instanceof dc?g=c:(g=new dc,tb(c,(l,h)=>{var m=g,w=m.j++;l=Zb(h,l);m.g.push(w);m.h[w]=l}));const k=cc(g,a.h,"/pagead/gen_204?id="+b+"&");k&&("undefined"!==typeof f?Q(n,k,f):Q(n,k))}catch(g){}}class fc{constructor(){this.h="http:"===R.location.protocol?"http:":"https:";this.g=Math.random()}};let gc=null;var hc=()=>{const a=n.performance;return a&&a.now&&a.timing?Math.floor(a.now()+a.timing.navigationStart):Date.now()},ic=()=>{const a=n.performance;return a&&a.now?a.now():null};class jc{constructor(a,b){var c=ic()||hc();this.label=a;this.type=b;this.value=c;this.duration=0;this.uniqueId=Math.random();this.slotId=void 0}};const U=n.performance,kc=!!(U&&U.mark&&U.measure&&U.clearMarks),V=ka(()=>{var a;if(a=kc){var b;if(null===gc){gc="";try{a="";try{a=n.top.location.hash}catch(c){a=n.location.hash}a&&(gc=(b=a.match(/\bdeid=([\d,]+)/))?b[1]:"")}catch(c){}}b=gc;a=!!b.indexOf&&0<=b.indexOf("1337")}return a});function lc(a){a&&U&&V()&&(U.clearMarks(`goog_${a.label}_${a.uniqueId}_start`),U.clearMarks(`goog_${a.label}_${a.uniqueId}_end`))} 
class mc{constructor(){var a=X;this.h=[];this.i=a||n;let b=null;a&&(a.google_js_reporting_queue=a.google_js_reporting_queue||[],this.h=a.google_js_reporting_queue,b=a.google_measure_js_timing);this.g=V()||(null!=b?b:1>Math.random())}start(a,b){if(!this.g)return null;a=new jc(a,b);b=`goog_${a.label}_${a.uniqueId}_start`;U&&V()&&U.mark(b);return a}end(a){if(this.g&&"number"===typeof a.value){a.duration=(ic()||hc())-a.value;var b=`goog_${a.label}_${a.uniqueId}_end`;U&&V()&&U.mark(b);!this.g||2048<this.h.length|| 
this.h.push(a)}}};function nc(a){let b=a.toString();a.name&&-1==b.indexOf(a.name)&&(b+=": "+a.name);a.message&&-1==b.indexOf(a.message)&&(b+=": "+a.message);if(a.stack){a=a.stack;try{-1==a.indexOf(b)&&(a=b+"\n"+a);let c;for(;a!=c;)c=a,a=a.replace(/((https?:\/..*\/)[^\/:]*:\d+(?:.|\n)*)\2/,"$1");b=a.replace(/\n */g,"\n")}catch(c){}}return b} 
function oc(a,b,c){let d,e;try{a.g&&a.g.g?(e=a.g.start(b.toString(),3),d=c(),a.g.end(e)):d=c()}catch(f){c=!0;try{lc(e),c=a.o(b,new cb(f,{message:nc(f)}),void 0,void 0)}catch(g){a.l(217,g)}if(c){let g,k;null==(g=window.console)||null==(k=g.error)||k.call(g,f)}else throw f;}return d}function pc(a,b){var c=qc;return(...d)=>oc(c,a,()=>b.apply(void 0,d))} 
class rc{constructor(){var a=sc;this.i=Y;this.h=null;this.o=this.l;this.g=void 0===a?null:a;this.j=!1}pinger(){return this.i}l(a,b,c,d,e){e=e||"jserror";let f;try{const D=new dc;var g=D;g.g.push(1);g.h[1]=Zb("context",a);b.error&&b.meta&&b.id||(b=new cb(b,{message:nc(b)}));if(b.msg){g=D;var k=b.msg.substring(0,512);g.g.push(2);g.h[2]=Zb("msg",k)}var l=b.meta||{};b=l;if(this.h)try{this.h(b)}catch(I){}if(d)try{d(b)}catch(I){}d=D;l=[l];d.g.push(3);d.h[3]=l;d=n;l=[];b=null;do{var h=d;if(sb(h)){var m= 
h.location.href;b=h.document&&h.document.referrer||null}else m=b,b=null;l.push(new Yb(m||""));try{d=h.parent}catch(I){d=null}}while(d&&h!=d);for(let I=0,Lb=l.length-1;I<=Lb;++I)l[I].depth=Lb-I;h=n;if(h.location&&h.location.ancestorOrigins&&h.location.ancestorOrigins.length==l.length-1)for(m=1;m<l.length;++m){var w=l[m];w.url||(w.url=h.location.ancestorOrigins[m-1]||"",w.G=!0)}var x=l;let na=new Yb(n.location.href,!1);h=null;const Ea=x.length-1;for(w=Ea;0<=w;--w){var E=x[w];!h&&Wb.test(E.url)&&(h= 
E);if(E.url&&!E.G){na=E;break}}E=null;const Ic=x.length&&x[Ea].url;0!=na.depth&&Ic&&(E=x[Ea]);f=new Xb(na,E);if(f.h){x=D;var J=f.h.url||"";x.g.push(4);x.h[4]=Zb("top",J)}var Fa={url:f.g.url||""};if(f.g.url){var Ga=f.g.url.match(jb),W=Ga[1],Mb=Ga[3],Nb=Ga[4];J="";W&&(J+=W+":");Mb&&(J+="//",J+=Mb,Nb&&(J+=":"+Nb));var Ob=J}else Ob="";W=D;Fa=[Fa,{url:Ob}];W.g.push(5);W.h[5]=Fa;ec(this.i,e,D,this.j,c)}catch(D){try{ec(this.i,e,{context:"ecmserr",rctx:a,msg:nc(D),url:f&&f.g.url},this.j,c)}catch(na){}}return!0}} 
;let Y,qc;if(Ub&&!sb(T)){let a="."+P.domain;try{for(;2<a.split(".").length&&!sb(T);)P.domain=a=a.substr(a.indexOf(".")+1),T=window.parent}catch(b){}sb(T)||(T=window)}const X=T,sc=new mc;var tc=()=>{if(!X.google_measure_js_timing){var a=sc;a.g=!1;a.h!=a.i.google_js_reporting_queue&&(V()&&Array.prototype.forEach.call(a.h,lc,void 0),a.h.length=0)}};Y=new fc;"number"!==typeof X.google_srt&&(X.google_srt=Math.random());var uc=Y,vc=X.google_srt;0<=vc&&1>=vc&&(uc.g=vc);qc=new rc; 
qc.h=a=>{const b=Ab;0!==b&&(a.jc=String(b),a.shv=Bb(b))};qc.j=!0;"complete"==X.document.readyState?tc():sc.g&&O(X,"load",()=>{tc()});var wc=(a,b)=>pc(a,b);var xc=(a,b)=>{b=H(a,2)||b;if(!b)return"";if(K(a,13))return b;const c=/[?&]adurl=([^&]+)/.exec(b);if(!c)return b;const d=[b.slice(0,c.index+1)];Ca(L(a,4,null),(e,f)=>{d.push(encodeURIComponent(f)+"="+encodeURIComponent(e)+"&")});d.push(b.slice(c.index+1));return d.join("")},yc=(a,b)=>{b=void 0===b?[]:b;b=0<b.length?b:Vb("data-asoch-targets");a=L(a,1,Sa);const c=[];for(let k=0;k<b.length;++k){var d=b[k].getAttribute("data-asoch-targets"),e=d.split(","),f=!0;for(let l of e)if(!a.has(l)){f=!1;break}if(f){f= 
a.get(e[0]);for(d=1;d<e.length;++d){var g=a.get(e[d]);f=Oa(f).m();g=g.m();const l=Math.max(f.length,g.length);for(let h=0;h<l;++h)null==f[h]&&(f[h]=g[h]);f=new Sa(f)}e=L(f,4,null);null!=F(f,5)&&e.set("nb",G(f,5,0).toString());c.push({element:b[k],data:f})}else ec(Y,"gdn-asoch",{type:1,data:d},!0,void 0,void 0)}return c},zc=(a,b,c,d)=>{var e=xc(b,c);if(0<e.length){if(null!=F(b,18)){c=609===d;var f=c?1:void 0!==P.featurePolicy&&P.featurePolicy.allowsFeature("conversion-measurement")?3:2;var g=f.toString(); 
f=encodeURIComponent("nis");g=encodeURIComponent(String(g));e=e.replace("?","?"+f+"="+g+"&");if(c||!a.hasAttribute("impressiondata"))c=N(b,Pa,18),a.setAttribute("conversiondestination",H(c,2)),a.setAttribute("impressiondata",H(c,1)),a.setAttribute("reportingorigin",H(c,3)),a.setAttribute("impressionexpiry",H(c,4))}u(a,Sb(e,Tb(d)));a.target||(a.target=null!=F(b,11)?H(b,11):"_top")}},Ac=a=>{var b=void 0===b?"":b;for(const d of a){a=d.data;var c=0===b.length?!1:d.element.matches(b);"A"!=d.element.tagName|| 
K(a,1)||c||(c=d.element,zc(c,a,c.href,609))}},Bc=a=>{const b=window.oneAfmaInstance;if(b)for(const c of a)if((a=c.data)&&null!=F(a,8)&&(a=H(N(a,Ua,8),4))){b.fetchAppStoreOverlay(a);break}},Cc=(a,b)=>{b=void 0===b?500:b;const c=[],d=[];for(var e of a)(a=e.data)&&null!=F(a,12)&&(d.push(N(a,Ta,12)),c.push(N(a,Ta,12).s()));e=(f,g)=>{if(g)for(const k of d)k.H(g[k.s()]||!1)};a=window.oneAfmaInstance;for(const f of c)a.canOpenAndroidApp(f,e,()=>{},b)},Ec=(a,b,c,d,e)=>{if(a||!b||null==F(b,12))return!1;const f= 
N(b,Ta,12).s();a="";if(0<Ka(b).length)for(const g of Ka(b))a+=H(g,2)+" "+g.s()+" ";if(K(N(b,Ta,12),2))return Dc({id:"gmob-apps",event:"och-open-android-app-before-click",deepLinks:a,appId:f,isDeepLinkPath:!1,exptIds:e}),d.click(c),d.openAndroidApp(f),setTimeout(()=>{var g={id:"gmob-apps",event:"och-open-android-app",appId:f,isDeepLinkPath:!1,exptIds:e};Z(rb(qb(),g))},1E3),!0;Dc({id:"gmob-apps",event:"och-open-android-app-validated-false",deepLinks:a,appId:f,isDeepLinkPath:!1,exptIds:e});return!1}, 
Fc=(a,b,c,d,e,f)=>{if(!c||null==F(c,8))return!1;const g=N(c,Ua,8);let k=H(g,2);Ca(L(c,10,null),(l,h)=>{var m=k;h=encodeURIComponent(h);const w=encodeURIComponent(l);l=new RegExp("[?&]"+h+"=([^&]+)");const x=l.exec(m);console.log(x);h=h+"="+w;k=x?m.replace(l,x[0].charAt(0)+h):m.replace("?","?"+h+"&")});Dc({id:"gmob-apps",event:"och-try-u2-redirect",appId:H(g,4)||"",isIos:a,isDeepLinkPath:!1,exptIds:f});c=e.openIntentOrNativeApp;if(a&&(null==b?0:K(b,6)))c=e.openSKOverlayWithUrl;else if(null==b?0:K(b, 
1))(null==b?0:K(b,5))?c=l=>e.openStoreOverlay(l,()=>e.openSKOverlayWithUrl(k)):c=e.openStoreOverlay;return e.redirectForStoreU2({clickUrl:d,trackingUrl:H(g,3),finalUrl:k,pingFunc:e.click,openFunc:c})},Gc=(a,b)=>{b=void 0===b?null:b;if(null!==b){const c=new aa({url:a});if(c.i&&c.h||c.l)return b(p(c,"&act=1&ri=1")),q(c,1)}else return b=new aa({url:a}),b.i&&b.h||b.l?navigator.sendBeacon?navigator.sendBeacon(p(b,"&act=1&ri=1"),"")?q(b,1):q(b,2):q(b,0):a;return a},Dc=a=>{Z(rb(qb(),a))},Z=(a,b)=>{(void 0=== 
b||b)&&R.fetch?R.fetch(a,{method:"GET",keepalive:!0,mode:"no-cors"}).then(c=>{c.ok||Q(R,a)}):Q(R,a)};var Jc=class extends z{constructor(){super();B(this,void 0,Hc)}},Hc=[6];const Kc="platform platformVersion architecture model uaFullVersion bitness".split(" ");var Lc=()=>{var a=window;return a.navigator&&a.navigator.userAgentData&&"function"===typeof a.navigator.userAgentData.getHighEntropyValues?a.navigator.userAgentData.getHighEntropyValues(Kc).then(b=>{var c=new Jc;c=M(c,1,b.platform);c=M(c,2,b.platformVersion);c=M(c,3,b.architecture);c=M(c,4,b.model);c=M(c,5,b.uaFullVersion);return M(c,9,b.bitness)}):null};function Mc(a){for(const b of a)if("A"==b.element.tagName){a=b.element;const c=b.data;null!=F(c,2)||M(c,2,a.href)}}function Nc(a,b){return fa(a,c=>c.element===b)}function Oc(a){xb(wc(556,()=>{new Pc(a||{})}))} 
function Qc(a,b,c,d){if(!K(d,13)){var e=window;e.pawsig&&"function"===typeof e.pawsig.clk&&e.pawsig.clk(c);e=c.href;var f=/[?&]adurl=([^&]+)/.exec(e);e=f?[e.slice(0,f.index),e.slice(f.index)]:[e,""];for(u(c,Sb(e[0],Tb(557)));!c.id;)if(f="asoch-id-"+la(),!P.getElementById(f)){c.id=f;break}f=c.id;"function"===typeof window.xy&&window.xy(b,c,P.body);"function"===typeof window.mb&&window.mb(c);"function"===typeof window.bgz&&window.bgz(f);"function"===typeof window.ja&&window.ja(f,d?G(d,5,0):0);a.i&& 
"function"===typeof window.ss&&(a.F?window.ss(f,1,a.i):window.ss(a.i,1));0<e.length&&(a=0<a.A.length?c.href+"&uach="+encodeURIComponent(a.A)+e[1]:c.href+e[1],u(c,Sb(a,Tb(557))))}}async function Rc(a,b,c,d){let e="";if(window.oneAfmaInstance){const f=window.oneAfmaInstance;e=await f.appendClickSignalsAsync(c.href)||"";if(a.B){const {modifiedUrl:g,error:k}=await f.appendNativeAdViewSignals(e);k||(e=g)}}Sc(a,b,c,d,e)} 
function Sc(a,b,c,d,e){const f=K(a.h,2),g=f&&Date.now()-a.D>a.K;if(window.oneAfmaInstance){b.preventDefault?b.preventDefault():b.returnValue=!1;var k=window.oneAfmaInstance;b=k.logScionEventAndAddParam(e);if(!Ec(a.l,d,b,k,a.o)&&!Fc(a.l,a.j,d,b,k,a.o)){c=a.L;e=a.l;var l=a.o;const h=K(d,15),m=!/[?&]dsh=1(&|$)/.test(b)&&/[?&]ae=1(&|$)/.test(b);!f||!g||h&&m||(b=Gc(b,k.click));b&&b.startsWith("intent:")?(k.openIntentOrNativeApp(b),d={id:"gmob-apps",event:"och-open-intent-or-native-app",appId:null!=F(d, 
8)&&H(N(d,Ua,8),4)||"",isIos:e,isDeepLinkPath:!1,exptIds:l},Z(rb(qb(),d))):c?k.openChromeCustomTab(b):k.openSystemBrowser(b,{useFirstPackage:!0,useRunningProcess:!0})}}else g&&(d=Gc(c.href),d!==c.href&&u(c,Sb(d,Tb(599))));g&&(a.D=Date.now());Db(a.C)} 
var Pc=class{constructor(a){this.l=ra||pa||sa||qa;var b=Vb("data-asoch-meta");if(1!==b.length)ec(Y,"gdn-asoch",{type:2,data:b.length},!0,void 0,void 0);else{this.C=70;this.h=new Wa(JSON.parse(b[0].getAttribute("data-asoch-meta"))||[]);this.J=a["extra-meta"]?new Wa(JSON.parse(a["extra-meta"])):null;this.B=!1;this.j=a["ios-store-overlay-config"]?new Xa(JSON.parse(a["ios-store-overlay-config"])):null;this.L="true"===a["use-cct-over-browser"];this.o=a["expt-ids"]||"";this.A="";b=Lc();null!=b&&b.then(d=> 
{this.A=JSON.stringify(d.h&&d.m(),Na)});this.g=yc(this.h);this.K=Number(a["async-click-timeout"])||300;this.M=Number(a["deeplink-and-android-app-validation-timeout"])||500;this.D=-Infinity;this.i=H(this.h,5)||"";this.F=K(this.h,11);this.J&&(this.F=K(this.J,11));this.v=this.u=null;K(this.h,3)||(Ac(this.g),M(this.h,3,!0));Mc(this.g);!this.l&&window.oneAfmaInstance&&Cc(this.g,this.M);var c;if(window.oneAfmaInstance&&(null==(c=this.j)?0:K(c,2)))switch(a=()=>{const d=G(this.j,4,0);0<d?n.setTimeout(()=> 
{Bc(this.g)},d):Bc(this.g)},G(this.j,3,0)){case 1:window.oneAfmaInstance.runOnOnShowEvent(a);break;case 2:yb(a);break;default:Bc(this.g)}O(P,"click",wc(557,d=>{a:if(!d.defaultPrevented||this.u===d){for(var e,f,g=d.target;(!e||!f)&&g;){f||"A"!=g.tagName||(f=g);var k=g.hasAttribute("data-asoch-targets");!e&&("A"==g.tagName||k)&&(k=k&&"true"===g.getAttribute("data-asoch-is-dynamic")?yc(this.h,[g]):this.g,k=Nc(k,g))&&(e=k.data);g=g.parentElement}if(g=e&&!K(e,1)){if(d.defaultPrevented){var l=f;f=e;if(this.u=== 
d&&this.v){var h=new Qa(this.v);e=H(f,9);g="";switch(G(h,4,1)){case 2:if(G(h,2,0))g="blocked_fast_click";else if(H(h,1)||H(h,7))g="blocked_border_click";break;case 3:h=P,h=h.getElementById?h.getElementById("common_15click_anchor"):null,"function"===typeof window.copfcChm&&h&&(f=Oa(f),M(f,5,12),L(f,4,null).set("nb",(12).toString()),(g=Nc(this.g,h))?g.data=f:this.g.push({element:h,data:f}),l&&(Qc(this,d,l,f),M(f,2,l.href)),window.copfcChm(d,xc(f,h.href))),g="onepointfiveclick_first_click"}e&&g&&Z(e+ 
"&label="+g,!1);Db(this.C)}break a}k=e;for(h of F(k,6))Z(h)}if(f&&g){e=g?e:null;(h=Nc(this.g,f))?h=h.data:(h=new Sa,M(h,2,f.href),M(h,11,f.target||"_top"),this.g.push({element:f,data:h}));zc(f,e||h,H(h,2),557);Qc(this,d,f,e);for(l of F(this.h,17)){let m;h=l;g=P.body;k={};"function"===typeof window.CustomEvent?m=new CustomEvent(h,k):(m=document.createEvent("CustomEvent"),m.initCustomEvent(h,!!k.bubbles,!!k.cancelable,k.detail));g.dispatchEvent(m)}K(this.h,16)||this.B?Rc(this,d,f,e):(l="",window.oneAfmaInstance&& 
(l=window.oneAfmaInstance.appendClickSignals(f.href)),Sc(this,d,f,e,l))}}}),db);this.i&&"function"===typeof window.ss&&O(P.body,"mouseover",wc(626,()=>{window.ss(this.i,0)}),eb);a=window;a.googqscp&&"function"===typeof a.googqscp.registerCallback&&a.googqscp.registerCallback((d,e)=>{this.u=d;this.v=e})}}};var Tc=wc(555,a=>Oc(a));Ab=70;const Uc=Cb(70,document.currentScript);if(null==Uc)throw Error("JSC not found 70");const Vc={},Wc=Uc.attributes;for(let a=Wc.length-1;0<=a;a--){const b=Wc[a].name;0===b.indexOf("data-jcp-")&&(Vc[b.substring(9)]=Wc[a].value)}Tc(Vc);}).call(this);
