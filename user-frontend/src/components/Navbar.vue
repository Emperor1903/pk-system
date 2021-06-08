<template>
<el-row type="flex" class="row-bg" justify="center">
  <el-col :span="16">
    <el-menu
      :default-active="activeIndex"
      class="el-menu"
      mode="horizontal"
      @select="handleSelect"
      background-color="#409EFF"
      text-color="#ffffff"
      active-text-color="#fffff">
      
      <el-menu-item class="menu-item" index="1">Trang chủ</el-menu-item>
      <el-menu-item index="2">Tìm kiếm</el-menu-item>
      <el-menu-item index="3">Hướng dẫn</el-menu-item>
<<<<<<< HEAD
    </el-menu>
    
  </el-col>
</el-row>
=======
      <!-- <el-submenu index="2">
        <template slot="title">Workspace</template>
        <el-menu-item index="2-1">item one</el-menu-item>
        <el-menu-item index="2-2">item two</el-menu-item>
        <el-menu-item index="2-3">item three</el-menu-item>
        <el-submenu index="2-4">
          <template slot="title">item four</template>
          <el-menu-item index="2-4-1">item one</el-menu-item>
          <el-menu-item index="2-4-2">item two</el-menu-item>
          <el-menu-item index="2-4-3">item three</el-menu-item>
        </el-submenu>
      </el-submenu>
      <el-menu-item index="3" disabled>Info</el-menu-item>
      <el-menu-item index="4"
        ><a href="https://www.ele.me" target="_blank">Orders</a></el-menu-item
      > -->

      <el-menu-item fixed="right" align="right">
        <span>
          <el-button
            size="mini"
            type="primary"
            icon="el-icon-plus"
            @click="handleNewBookingTicket()"
            >Đăng ký khám
          </el-button>
        </span>
      </el-menu-item>
    </el-menu>
    <NewBookingTicketDialog
      :state="newBookingTicketState"
    ></NewBookingTicketDialog>
  </div>
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
</template>

<style>
.invisible-box {
    width: 400px;
}
</style>

<script>
export default {
<<<<<<< HEAD
    data() {
        return {
            activeIndex: "1",
            newLedgerState: {
                title: "Sổ khám",
                doc: "ledger",
                visible: false,
            },
            newBookingTicketState: {
                title: "Đămg kí khám chữa bệnh",
                doc: "bookingticket",
                visible: false,
            },
        };
=======
  components: {
    NewBookingTicketDialog: () => import("./NewBookingTicketDialog.vue"),
  },
  data() {
    return {
      activeIndex: "1",
      newLedgerState: {
        title: "Sổ khám",
        doc: "ledger",
        visible: false,
      },
      newBookingTicketState: {
        title: "Đămg kí khám chữa bệnh",
        doc: "bookingticket",
        visible: false,
      },
    };
  },
  methods: {
    async handleSelect(key, keyPath) {
      switch (key) {
        case "2":
          await this.$router.push("/search");
      }
    },
    handleNewLedger() {
      this.newLedgerState.visible = true;
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
    },
    methods: {
        handleSelect(key, keyPath) {
            console.log(key, keyPath);
        },
    },
};
</script>
