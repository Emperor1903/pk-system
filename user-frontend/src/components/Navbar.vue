<template>
  <div>
    <el-menu
      :default-active="activeIndex"
      class="el-menu-demo"
      mode="horizontal"
      @select="handleSelect"
      background-color="#545c64"
      text-color="#fff"
      active-text-color="#ffd04b"
    >
      <el-menu-item index="1">Trang chủ</el-menu-item>
      <el-menu-item index="2">Tìm kiếm</el-menu-item>
      <el-menu-item index="3">Hướng dẫn</el-menu-item>
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
</template>

<script>
export default {
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
    },
    handleNewBookingTicket() {
      this.newBookingTicketState.visible = true;
    },
  },
};
</script>