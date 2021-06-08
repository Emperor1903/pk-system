<template>
  <div id="app">
<<<<<<< HEAD
    <Navbar/>
    <router-view />
=======
    <Navbar />
    <el-col>
      <router-view />
    </el-col>
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
  </div>
</template>

<script>
import Navbar from "./components/Navbar.vue";

export default {
  name: "app",
  components: {
      Navbar: () => import("./components/Navbar"),
  },
};
</script>

