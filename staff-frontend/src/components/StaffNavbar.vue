<template>
  <el-menu
    :default-active="activeIndex"
    class="el-menu-vertical-demo"
    @select="handleSelect">
    <el-menu-item index="3">
      <i class="el-icon-info"></i>
      <span>Chi tiết phòng khám</span>
    </el-menu-item>
    <el-menu-item index="4">
      <i class="el-icon-info"></i>
      <span>Danh sách ca trực</span>
    </el-menu-item>    
    <el-menu-item index="9">
      <i class="el-icon-caret-left"></i>
      <span>Đăng xuất</span>
    </el-menu-item>

  </el-menu>
  
</template>

<script>
import {logout, getIdentity} from "../api/auth";
  
export default {
    data() {
        return {
            clinicId: "",
            activeIndex: "1",
        };
    },
    async mounted() {
        var user = await getIdentity();
        this.clinicId = user.clinic["$oid"];
    },
    methods: {
        handleSelect(key) {
            switch (key) {
            case "3":
                this.$router.push(`/doctor/${this.clinicId}`);
                break;
            case "4":
                this.$router.push(`/shift/${this.clinicId}`);
                break;                
            case "9":
                logout();
                location.reload();
                break;
            default:
                break;
      }
    },
  },
};
</script>
