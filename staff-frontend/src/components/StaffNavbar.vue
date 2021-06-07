<template>
  <el-menu
    :default-active="activeIndex"
    class="el-menu-vertical-demo"
    @open="handleOpen"
    @close="handleClose"
    @select="handleSelect">
    <el-menu-item index="3">
      <i class="el-icon-menu"></i>
      <span>Danh sách bác sĩ</span>
    </el-menu-item>    
    <el-menu-item index="9">
      <i class="el-icon-caret-left"></i>
      <span>Đăng xuất</span>
    </el-menu-item>    

  </el-menu>
  
</template>

<script>
import {logout} from "../api/auth";
  
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
        async handleSelect(key, keyPath) {
            switch (key) {
            case "3":
                await this.$router.push("/staff/doctors");
                break;
            case "9":
                await logout();
                location.reload();
                break;
            default:
                break;
      }
    },
  },
};
</script>
