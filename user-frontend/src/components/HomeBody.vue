<template>
<el-row type="flex" class="row-bg" justify="center">
  <el-col :span="8">
    <el-card class="box-card" shadow="never" :body-style="{ padding: '0px' }">
      <img src="../assets/femail-doctor.jpg" class="image"/>
      <div class="text-container">
        <span class="text">
          Chào mừng bạn đến với hệ thống đăng ký khám bệnh tự động. Hãy
          đăng ký khám bệnh hoàn toàn miễn phí ngay bây giờ!
        </span>
      </div>
      <br/>
      <el-button
        type="primary"
        class="button"
        @click="handleBooking">
        Bắt đầu đăng ký khám
      </el-button>
    </el-card>
  </el-col>
  <NewBookingTicketDialog
    :state="newBookingTicketState">
  </NewBookingTicketDialog>
  
</el-row>
</template>

<style>
.box-card {
    
}

.text-container {
    margin-top: 40px;
    margin-left: 40px;
    margin-right: 40px;        
}

.text {
    text-align: left;
    line-height: 1.8;    
}

.button {
    margin: 40px;
}

.image {
    width: 100%;
    display: block;
}
</style>

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
        handleBooking() {
            this.newBookingTicketState.visible = true;
        },
    }
}  
</script>
