import Vue from 'vue'
import App from './App.vue'
import router from './router'
import './index.css'
import InfiniteLoading from "vue-infinite-loading";
import VueAnalytics from "vue-analytics";
import {store} from '@/store/store'

Vue.config.productionTip = false
Vue.use(InfiniteLoading)
Vue.use(VueAnalytics, {
  id: 'G-HPLNHBX0YK',
  router
})

new Vue({
  router,
  store,
  render: function (h) { return h(App) }
}).$mount('#app')
