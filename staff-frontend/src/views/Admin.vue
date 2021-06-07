<template>
  <el-row type="flex">
    <el-col :span="4">
      <Navbar />
    </el-col>    
    <el-col>
      <router-view />
    </el-col>
  </el-row>
</template>

<script>
export default {
    components: { Navbar: () => import("../components/Navbar") },
};
</script>
