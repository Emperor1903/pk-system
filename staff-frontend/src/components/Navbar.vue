<template>
  <el-menu
    :default-active="activeIndex"
    class="el-menu-vertical-demo"
    @open="handleOpen"
    @close="handleClose"
    @select="handleSelect"
  >
    <el-menu-item index="1">
      <i class="el-icon-menu"></i>
      <span>Quản lý bệnh viện</span>
    </el-menu-item>
    <el-submenu index="2">
      <template slot="title">
        <i class="el-icon-menu"></i>
        <span>Danh mục</span>
      </template>
      <el-menu-item index="2-1">Chuyên môn</el-menu-item>
    </el-submenu>
  </el-menu>
</template>

<script>
export default {
  data() {
    return {
      activeIndex: "1",
    };
  },
  methods: {
    handleOpen(key, keyPath) {
      //   console.log(key, keyPath);
    },
    handleClose(key, keyPath) {
      //   console.log(key, keyPath);
    },
    handleSelect(key, keyPath) {
      switch (key) {
        case "1":
          this.$router.push(`/`);
          break;
        case "2-1":
          this.$router.push(`/specializations`);
          break;
      }
    },
  },
};
</script>