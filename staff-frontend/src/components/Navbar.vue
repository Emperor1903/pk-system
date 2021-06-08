<template>
  <el-menu
    :default-active="activeIndex"
    class="el-menu-vertical-demo"
    @open="handleOpen"
    @close="handleClose"
    @select="handleSelect">
    <el-menu-item index="1">
      <i class="el-icon-menu"></i>
      <span>Danh sách bệnh viện</span>
    </el-menu-item>
    <el-menu-item index="2">
      <i class="el-icon-menu"></i>
      <span>Danh sách phòng khám</span>
    </el-menu-item>
    <el-menu-item index="3">
      <i class="el-icon-menu"></i>
      <span>Danh sách bác sĩ</span>
    </el-menu-item>    
    <el-menu-item index="4">
      <i class="el-icon-menu"></i>
      <span>Danh sách chuyên môn</span>
    </el-menu-item>
    <el-menu-item index="5">
      <i class="el-icon-menu"></i>
      <span>Danh sách tỉnh/thành phố</span>
    </el-menu-item>
    <el-menu-item index="6">
      <i class="el-icon-menu"></i>
      <span>Danh sách ca trực</span>
    </el-menu-item>
    <el-menu-item index="7">
      <i class="el-icon-menu"></i>
      <span>Danh sách quản trị viên</span>
    </el-menu-item>
    <el-menu-item index="8">
      <i class="el-icon-menu"></i>
      <span>Danh sách nhân viên</span>
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
            case "1":
                await this.$router.push("/");
                break;
            case "2":
                await this.$router.push("/clinics");
                break;
            case "3":
                await this.$router.push("/doctors");
                break;
            case "4":
                await this.$router.push("/specializations");
                break;
            case "5":
                await this.$router.push("/provinces");
                break;
            case "6":
                await this.$router.push("/shifts");
                break;
            case "7":
                await this.$router.push("/admins");
                break;
            case "8":
                await this.$router.push("/staffs");
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
