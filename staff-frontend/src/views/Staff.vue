<template>
  <el-row type="flex">
    <el-col :span="4">
      <StaffNavbar />
    </el-col>    
    <el-col>
      <router-view />
    </el-col>
  </el-row>
</template>

<script>
export default {
    components: { StaffNavbar: () => import("../components/StaffNavbar") },
};
</script>
