<template>
    <el-dialog
    :title="state.title"
    :visible.sync="isVisible"
    width="30%"
    :before-close="handleClose">
    <span>{{ this.state.data }}/span>
    <span slot="footer" class="dialog-footer">
        <el-button @click="dialogVisible = false">Cancel</el-button>
        <el-button type="primary" @click="dialogVisible = false">Confirm</el-button>
    </span>
    </el-dialog>    
</template>

<script>
export default {
    name: "DeleteDialog",
    props: ['state'],
    data() {
        return {
            isVisible: true
        };
    },
    created: function() {
        console.log(this.state);
    },
    methods: {
        async handleClose(done) {
            try {
                await this.$confirm(`Bạn có chắc là sẽ xóa ${this.state.title} này không?`);
                done();
            } catch (err) {
                
            }
        }
    }
}
</script>