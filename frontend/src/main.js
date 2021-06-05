import Vue from 'vue'
import App from './App.vue'
import router from './router'
import './index.css'
import InfiniteLoading from "vue-infinite-loading";

Vue.config.productionTip = false
Vue.use(InfiniteLoading)

new Vue({
  router,
  render: function (h) { return h(App) }
}).$mount('#app')
