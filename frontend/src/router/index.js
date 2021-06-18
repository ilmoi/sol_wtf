import Vue from 'vue'
import VueRouter from 'vue-router'
import Feed from '../views/Feed'
import About from '../views/About'
import Tip from '../views/Tip'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'feed',
    component: Feed
  },
  {
    path: '/about',
    name: 'about',
    component: About
  },
  {
    path: '/tip',
    name: 'tip the dev',
    component: Tip
  }
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router
