function n(n,t,e,r){return new(e||(e=Promise))((function(o,i){function c(n){try{a(r.next(n))}catch(n){i(n)}}function u(n){try{a(r.throw(n))}catch(n){i(n)}}function a(n){var t;n.done?o(n.value):(t=n.value,t instanceof e?t:new e((function(n){n(t)}))).then(c,u)}a((r=r.apply(n,t||[])).next())}))}function t(n,t){var e,r,o,i,c={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:u(0),throw:u(1),return:u(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function u(i){return function(u){return function(i){if(e)throw new TypeError("Generator is already executing.");for(;c;)try{if(e=1,r&&(o=2&i[0]?r.return:i[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,i[1])).done)return o;switch(r=0,o&&(i=[2&i[0],o.value]),i[0]){case 0:case 1:o=i;break;case 4:return c.label++,{value:i[1],done:!1};case 5:c.label++,r=i[1],i=[0];continue;case 7:i=c.ops.pop(),c.trys.pop();continue;default:if(!(o=c.trys,(o=o.length>0&&o[o.length-1])||6!==i[0]&&2!==i[0])){c=0;continue}if(3===i[0]&&(!o||i[1]>o[0]&&i[1]<o[3])){c.label=i[1];break}if(6===i[0]&&c.label<o[1]){c.label=o[1],o=i;break}if(o&&c.label<o[2]){c.label=o[2],c.ops.push(i);break}o[2]&&c.ops.pop(),c.trys.pop();continue}i=t.call(n,c)}catch(n){i=[6,n],r=0}finally{e=o=0}if(5&i[0])throw i[1];return{value:i[0]?i[1]:void 0,done:!0}}([i,u])}}}var e=function(){return(e=Object.assign||function(n){for(var t,e=1,r=arguments.length;e<r;e++)for(var o in t=arguments[e])Object.prototype.hasOwnProperty.call(t,o)&&(n[o]=t[o]);return n}).apply(this,arguments)};function r(n,t){void 0===t&&(t=!1);var e=window.crypto.getRandomValues(new Uint32Array(1))[0],r="_".concat(e);return Object.defineProperty(window,r,{value:function(e){return t&&Reflect.deleteProperty(window,r),null==n?void 0:n(e)},writable:!1,configurable:!0}),e}function o(n,t){return void 0===t&&(t={}),function(n,t,e,r){return new(e||(e=Promise))((function(o,i){function c(n){try{a(r.next(n))}catch(n){i(n)}}function u(n){try{a(r.throw(n))}catch(n){i(n)}}function a(n){var t;n.done?o(n.value):(t=n.value,t instanceof e?t:new e((function(n){n(t)}))).then(c,u)}a((r=r.apply(n,t||[])).next())}))}(this,void 0,void 0,(function(){return function(n,t){var e,r,o,i,c={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:u(0),throw:u(1),return:u(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function u(i){return function(u){return function(i){if(e)throw new TypeError("Generator is already executing.");for(;c;)try{if(e=1,r&&(o=2&i[0]?r.return:i[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,i[1])).done)return o;switch(r=0,o&&(i=[2&i[0],o.value]),i[0]){case 0:case 1:o=i;break;case 4:return c.label++,{value:i[1],done:!1};case 5:c.label++,r=i[1],i=[0];continue;case 7:i=c.ops.pop(),c.trys.pop();continue;default:if(!((o=(o=c.trys).length>0&&o[o.length-1])||6!==i[0]&&2!==i[0])){c=0;continue}if(3===i[0]&&(!o||i[1]>o[0]&&i[1]<o[3])){c.label=i[1];break}if(6===i[0]&&c.label<o[1]){c.label=o[1],o=i;break}if(o&&c.label<o[2]){c.label=o[2],c.ops.push(i);break}o[2]&&c.ops.pop(),c.trys.pop();continue}i=t.call(n,c)}catch(n){i=[6,n],r=0}finally{e=o=0}if(5&i[0])throw i[1];return{value:i[0]?i[1]:void 0,done:!0}}([i,u])}}}(this,(function(o){return[2,new Promise((function(o,i){var c=r((function(n){o(n),Reflect.deleteProperty(window,"_".concat(u))}),!0),u=r((function(n){i(n),Reflect.deleteProperty(window,"_".concat(c))}),!0);window.__TAURI_IPC__(e({cmd:n,callback:c,error:u},t))}))]}))}))}function i(e){return n(this,void 0,void 0,(function(){return t(this,(function(n){return[2,o("plugin:solana-wallet|generate_mnemonic_phrase",e)]}))}))}function c(e){return n(this,void 0,void 0,(function(){return t(this,(function(n){return[2,o("plugin:solana-wallet|import_mnemonic",e)]}))}))}function u(e){return n(this,void 0,void 0,(function(){return t(this,(function(n){return[2,o("plugin:solana-wallet|delete_mnemonic",{publicKey:e})]}))}))}function a(){return n(this,void 0,void 0,(function(){return t(this,(function(n){return[2,o("plugin:solana-wallet|fetch_mnemonic_metadatas")]}))}))}Object.freeze({__proto__:null,transformCallback:r,invoke:o,convertFileSrc:function(n,t){void 0===t&&(t="asset");var e=encodeURIComponent(n);return navigator.userAgent.includes("Windows")?"https://".concat(t,".localhost/").concat(e):"".concat(t,"://").concat(e)}});export{u as deleteMnemonic,a as fetchMnemonicMetadatas,i as generateMnemonicPhrase,c as importMnemonic};
