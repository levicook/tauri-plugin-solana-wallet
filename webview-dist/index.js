function n(n,e,t,r){return new(t||(t=Promise))((function(o,c){function i(n){try{u(r.next(n))}catch(n){c(n)}}function a(n){try{u(r.throw(n))}catch(n){c(n)}}function u(n){var e;n.done?o(n.value):(e=n.value,e instanceof t?e:new t((function(n){n(e)}))).then(i,a)}u((r=r.apply(n,e||[])).next())}))}function e(n,e){var t,r,o,c,i={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return c={next:a(0),throw:a(1),return:a(2)},"function"==typeof Symbol&&(c[Symbol.iterator]=function(){return this}),c;function a(c){return function(a){return function(c){if(t)throw new TypeError("Generator is already executing.");for(;i;)try{if(t=1,r&&(o=2&c[0]?r.return:c[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,c[1])).done)return o;switch(r=0,o&&(c=[2&c[0],o.value]),c[0]){case 0:case 1:o=c;break;case 4:return i.label++,{value:c[1],done:!1};case 5:i.label++,r=c[1],c=[0];continue;case 7:c=i.ops.pop(),i.trys.pop();continue;default:if(!(o=i.trys,(o=o.length>0&&o[o.length-1])||6!==c[0]&&2!==c[0])){i=0;continue}if(3===c[0]&&(!o||c[1]>o[0]&&c[1]<o[3])){i.label=c[1];break}if(6===c[0]&&i.label<o[1]){i.label=o[1],o=c;break}if(o&&i.label<o[2]){i.label=o[2],i.ops.push(c);break}o[2]&&i.ops.pop(),i.trys.pop();continue}c=e.call(n,i)}catch(n){c=[6,n],r=0}finally{t=o=0}if(5&c[0])throw c[1];return{value:c[0]?c[1]:void 0,done:!0}}([c,a])}}}var t=function(){return(t=Object.assign||function(n){for(var e,t=1,r=arguments.length;t<r;t++)for(var o in e=arguments[t])Object.prototype.hasOwnProperty.call(e,o)&&(n[o]=e[o]);return n}).apply(this,arguments)};function r(n,e){void 0===e&&(e=!1);var t=window.crypto.getRandomValues(new Uint32Array(1))[0],r="_".concat(t);return Object.defineProperty(window,r,{value:function(t){return e&&Reflect.deleteProperty(window,r),null==n?void 0:n(t)},writable:!1,configurable:!0}),t}function o(n,e){return void 0===e&&(e={}),function(n,e,t,r){return new(t||(t=Promise))((function(o,c){function i(n){try{u(r.next(n))}catch(n){c(n)}}function a(n){try{u(r.throw(n))}catch(n){c(n)}}function u(n){var e;n.done?o(n.value):(e=n.value,e instanceof t?e:new t((function(n){n(e)}))).then(i,a)}u((r=r.apply(n,e||[])).next())}))}(this,void 0,void 0,(function(){return function(n,e){var t,r,o,c,i={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return c={next:a(0),throw:a(1),return:a(2)},"function"==typeof Symbol&&(c[Symbol.iterator]=function(){return this}),c;function a(c){return function(a){return function(c){if(t)throw new TypeError("Generator is already executing.");for(;i;)try{if(t=1,r&&(o=2&c[0]?r.return:c[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,c[1])).done)return o;switch(r=0,o&&(c=[2&c[0],o.value]),c[0]){case 0:case 1:o=c;break;case 4:return i.label++,{value:c[1],done:!1};case 5:i.label++,r=c[1],c=[0];continue;case 7:c=i.ops.pop(),i.trys.pop();continue;default:if(!((o=(o=i.trys).length>0&&o[o.length-1])||6!==c[0]&&2!==c[0])){i=0;continue}if(3===c[0]&&(!o||c[1]>o[0]&&c[1]<o[3])){i.label=c[1];break}if(6===c[0]&&i.label<o[1]){i.label=o[1],o=c;break}if(o&&i.label<o[2]){i.label=o[2],i.ops.push(c);break}o[2]&&i.ops.pop(),i.trys.pop();continue}c=e.call(n,i)}catch(n){c=[6,n],r=0}finally{t=o=0}if(5&c[0])throw c[1];return{value:c[0]?c[1]:void 0,done:!0}}([c,a])}}}(this,(function(o){return[2,new Promise((function(o,c){var i=r((function(n){o(n),Reflect.deleteProperty(window,"_".concat(a))}),!0),a=r((function(n){c(n),Reflect.deleteProperty(window,"_".concat(i))}),!0);window.__TAURI_IPC__(t({cmd:n,callback:i,error:a},e))}))]}))}))}function c(){return n(this,void 0,void 0,(function(){return e(this,(function(n){switch(n.label){case 0:return[4,o("plugin:solana-wallet|execute")];case 1:return n.sent(),[2]}}))}))}Object.freeze({__proto__:null,transformCallback:r,invoke:o,convertFileSrc:function(n,e){void 0===e&&(e="asset");var t=encodeURIComponent(n);return navigator.userAgent.includes("Windows")?"https://".concat(e,".localhost/").concat(t):"".concat(e,"://").concat(t)}});export{c as execute};
