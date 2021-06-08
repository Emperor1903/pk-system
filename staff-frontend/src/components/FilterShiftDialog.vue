<template>
<el-dialog
  :title="state.title"
  :visible.sync="state.visible"
  width="30%"
  v-loading="loading">
  <span>Tạo ca trực của tuần thiếp theo cho tất cả bác sĩ?</span>  
  <span slot="footer" class="dialog-footer">
    <el-button @click="handleCancel">Không</el-button>
    <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
  </span>
</el-dialog>
</template>

<script>
import {createShifts} from "../api/index.js";
  
export default {
    name: "AutoCreateShiftDialog",
    props: ['state'],
    data() {
        return {
            loading: false
        };
    },
    methods: {
        async handleConfirm() {
            this.loading = true;
            try {
//                await createShifts();
            } finally {
                this.state.visible = false;
                this.loading = false;
                this.$emit("confirm");
            }
        },
        async handleCancel() {
            this.state.visible = false;
        },
    }
}
</script>
