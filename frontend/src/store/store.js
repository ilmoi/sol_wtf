import Vuex from 'vuex'
import Vue from "vue";

Vue.use(Vuex)

export const store = new Vuex.Store({
  state: {
    theme: localStorage.getItem('theme') || 'light'
  },
  mutations: {
    toggleTheme (state) {
      if (localStorage.theme === 'light' || (!('theme' in localStorage))) {
        localStorage.setItem('theme', 'dark')
        state.theme = 'dark'
      } else {
        localStorage.setItem('theme', 'light')
        state.theme = 'light'
      }
    }
  }
})

