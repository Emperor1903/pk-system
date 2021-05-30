<template>
  <div id="app">
    <el-row>
      <el-col :span="4"><Navbar></Navbar></el-col>
      <el-col :span="20"><router-view /></el-col>
    </el-row>
  </div>
</template>

<style>
.app {
    word-break: normal    
}
</style>

<script>
export default {
  components: { Navbar: () => import("./components/Navbar") },
};
</script>
