<template>
  <div :class="this.$store.state.theme">
    <div class="text-black dark:text-white bg-white dark:bg-black">
      <NavBar/>
      <button class="bg-black dark:bg-white bubble-bg" @click="toggleTheme">
        <svg class="w-6 fill-current text-white dark:text-black" version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
            viewBox="0 0 512 512" style="enable-background:new 0 0 512 512;" xml:space="preserve">
        <g>
          <g>
            <path d="M255.996,106c-82.705,0-150,67.295-150,150s67.295,150,150,150s150-67.295,150-150S338.701,106,255.996,106z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M411.696,346c-15.6,27.299-38.699,50.099-66,66l90.3,24.3L411.696,346z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M255.996,0l-46.199,82.299C224.497,78.1,240.095,76,255.996,76s31.5,2.1,46.2,6.299L255.996,0z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M75.996,76l24.3,90c15.601-27.301,38.101-49.801,65.4-65.7L75.996,76z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M82.297,209.5l-82,46.5l82,46.5c-4.2-14.7-6.301-30.3-6.301-46.5C75.996,239.799,78.096,224.2,82.297,209.5z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M100.296,346l-24.3,90.3l90.3-24.3C138.996,396.099,115.897,373.299,100.296,346z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M255.996,436c-16.199,0-31.8-2.1-46.8-6.301l46.8,82.301l46.8-82.301C287.796,433.899,272.195,436,255.996,436z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M346.159,100.452c27.14,15.795,49.756,38.43,65.514,65.596l24.426-90.004L346.159,100.452z"/>
          </g>
        </g>
          <g>
          <g>
            <path d="M511.703,256.146l-81.951-46.39c3.941,14.786,6.244,30.233,6.244,46.244c0,16.135-2.333,31.703-6.334,46.593
              L511.703,256.146z"/>
          </g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
          <g>
        </g>
      </svg>
      </button>
      <div>
        <router-view/>
      </div>
    </div>
  </div>
</template>

<script>
import NavBar from "@/components/NavBar";

export default {
  components: {NavBar},
  methods: {
    toggleTheme() {
      this.$store.commit('toggleTheme')
    }
  }
}
</script>

<style>
@font-face {
  font-family: "atari";
  src: local("atari"), url('./assets/Atari.ttf') format("truetype");
}

.bubble-bg {
  @apply bottom-2 right-2 fivefifty:bottom-8 fivefifty:right-8
  fixed flex justify-center items-center;
  width: 50px;
  height: 50px;
  /*bottom: 50px;*/
  /*right: 50px;*/
  border-radius: 50%;
  outline:0 !important;
}

.bubble-bg:focus {
  outline:0 !important;
}
</style>