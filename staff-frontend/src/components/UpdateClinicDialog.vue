<template>
<el-dialog
  v-if="fields"
  v-loading="loading"
  :title="state.title"
  :visible.sync="state.visible"
  width="50%">
  <Form :state="state" :fields="fieldList" @confirm="handleConfirm"/>
</el-dialog>
</template>

<script>
import {updateClinic, searchClinic, searchSpecialization} from "../api/admin.js";

export default {
    name: "UpdateClinicDialog",
    props: ["state"],
    components: {
        Form: () => import("./FormWithImage.vue"),
    },
    data() {
        return {
            loading: false,
            fields: {
                "name": {
                    name: "Tên phòng khám",
                    type: "input",
                    value: "",
                },
                "time_desc": {
                    name: "Thời gian mở cửa",
                    type: "area",
                    row: 2,
                    value: ""
                },
                "desc": {
                    name: "Giới thiệu",
                    type: "area",
                    row: 4,
                    value: ""
                },
                "hospital": {
                    name: "Bệnh viện",
                    type: "search",
                    value: "",
                    query: async function(keyword, callback) {
                        let response = await searchHospital(keyword, null, 0);
                        let data = response.data.map(i => {
                            if(i)
                                return {
                                    value: i.name,
                                    id: i._id,
                                    field: "hospital",
                                }
                        })
                        callback(data);
                    }
                },
                "address": {
                    name: "Địa chỉ",
                    type: "input",
                    value: "",                    
                },
                "phone_num": {
                    name: "Số điện thoại",
                    type: "input",
                    value: "",
                },
                "images": {
                    type: "image",
                    value: [],
                }
            },
        }
    },
    computed: {
        fieldList() {
            var list = [];
            for (var key in this.fields) {
                var t = { ...this.fields[key] };
                t.field = key;
                list.push(t);
            }
            return list;
        },
        visible() {
            return this.state.visible;
        }
    },
    watch: {
        visible: async function(val) {
            this.fields = {...this.fields};
        }
    },
    mounted() {

    },
    methods: {
        async handleConfirm(data) {
            if (data) {
                this.loading = true;
                console.log(data);
                try {
                    await updateClinic(
                        data._id,
                        data.name,
                        data.desc,
                        data.address,                
                        data.time_desc,
                        data.phone_num,
                        data.hospital,
                        data.images.map(i => i.url)
                    );
                } finally {
                    this.loading = false;
                }
                this.$emit('confirm');
                this.state.visible = false;
            }
        },
    },
}
</script>
