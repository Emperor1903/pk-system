<template>
<el-dialog
  :title="state.title"
  :visible.sync="state.visible"
  width="30%">
  <span>Bạn có chắc là sẽ xóa {{this.state.title}} này không?</span>  
  <span slot="footer" class="dialog-footer">
    <el-button @click="handleCancel">Không</el-button>
    <el-button type="primary" @click="handleConfirm">Xác nhận</el-button>
  </span>
</el-dialog>    
</template>

<script>
import {deleteDocument} from "../api/admin.js";
  
export default {
    name: "DeleteDialog",
    props: ['state'],
    data() {
        return {
            
        };
    },
    methods: {
        async handleConfirm() {
            await deleteDocument(this.state.doc, this.state.data["_id"]);
            this.state.visible = false;
            this.$emit('confirm');
        },
        async handleCancel() {
            this.state.visible = false;
        },
    }
}
</script>
