parcelRequire=function(e,r,n,t){var i="function"==typeof parcelRequire&&parcelRequire,o="function"==typeof require&&require;function u(n,t){if(!r[n]){if(!e[n]){var f="function"==typeof parcelRequire&&parcelRequire;if(!t&&f)return f(n,!0);if(i)return i(n,!0);if(o&&"string"==typeof n)return o(n);var c=new Error("Cannot find module '"+n+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[n][1][r]||r},p.cache={};var l=r[n]=new u.Module(n);e[n][0].call(l.exports,p,l,l.exports,this)}return r[n].exports;function p(e){return u(p.resolve(e))}}u.isParcelRequire=!0,u.Module=function(e){this.id=e,this.bundle=u,this.exports={}},u.modules=e,u.cache=r,u.parent=i,u.register=function(r,n){e[r]=[function(e,r){r.exports=n},{}]};for(var f=0;f<n.length;f++)u(n[f]);if(n.length){var c=u(n[n.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=c:"function"==typeof define&&define.amd?define(function(){return c}):t&&(this[t]=c)}return u}({"269q":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.Dashboard=void 0;var e,t=o(require("antd/es/layout")),r=o(require("react")),n=require("react-router-dom");function o(e){return e&&e.__esModule?e:{default:e}}function u(e){return(u="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(e){return typeof e}:function(e){return e&&"function"==typeof Symbol&&e.constructor===Symbol&&e!==Symbol.prototype?"symbol":typeof e})(e)}function a(e,t){if(!(e instanceof t))throw new TypeError("Cannot call a class as a function")}function l(e,t){for(var r=0;r<t.length;r++){var n=t[r];n.enumerable=n.enumerable||!1,n.configurable=!0,"value"in n&&(n.writable=!0),Object.defineProperty(e,n.key,n)}}function i(e,t,r){return t&&l(e.prototype,t),r&&l(e,r),e}function c(e,t){return!t||"object"!==u(t)&&"function"!=typeof t?f(e):t}function f(e){if(void 0===e)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return e}function s(e){return(s=Object.setPrototypeOf?Object.getPrototypeOf:function(e){return e.__proto__||Object.getPrototypeOf(e)})(e)}function p(e,t){if("function"!=typeof t&&null!==t)throw new TypeError("Super expression must either be null or a function");e.prototype=Object.create(t&&t.prototype,{constructor:{value:e,writable:!0,configurable:!0}}),t&&y(e,t)}function y(e,t){return(y=Object.setPrototypeOf||function(e,t){return e.__proto__=t,e})(e,t)}var b=function(){var e=(new Date).getFullYear();return r.default.createElement(t.default.Footer,{style:{textAlign:"center"}},"JSA ©",e," by jarrysix")},d=(0,n.withRouter)(e=function(e){function t(){return a(this,t),c(this,s(t).apply(this,arguments))}return p(t,r.default.Component),i(t,[{key:"start",value:function(){this.props.history.push("/domain")}},{key:"render",value:function(){return r.default.createElement(r.default.Fragment,null,r.default.createElement("center",null,r.default.createElement("a",{href:"javascript:void(0)",onClick:this.start.bind(this)},"开使使用")),r.default.createElement("br",null)," ",r.default.createElement("br",null),r.default.createElement(b,null))}}]),t}())||e;exports.Dashboard=d;
},{"antd/es/layout":"ieRz","react":"HdMw","react-router-dom":"rx6s"}],"AeTR":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.default=void 0;var e=require("./Dashboard"),r=e.Dashboard;exports.default=r;
},{"./Dashboard":"269q"}]},{},[], null)
//# sourceMappingURL=board.bbf1ea24.map