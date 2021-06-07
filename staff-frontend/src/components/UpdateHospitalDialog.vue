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
import {updateHospital, searchHospital, searchSpecialization} from "../api/admin.js";

export default {
    name: "UpdateHospitalDialog",
    props: ["state"],
    components: {
        Form: () => import("./FormWithImage.vue"),
    },
    data() {
        return {
            loading: false,
            fields: {
                "name": {
                    name: "Tên bệnh viện",
                    type: "input",
                    value: "",
                },
                "desc": {
                    name: "Giới thiệu",
                    type: "area",
                    row: 4,
                    value: ""
                },
                "province": {
                    name: "Tỉnh thành",
                    type: "search",
                    value: "",
                    query: async function(keyword, callback) {
                        let response = await searchProvince(keyword, 0);
                        let data = response.data.map(i => {
                            if(i)
                                return {
                                    value: i.name,
                                    id: i._id,
                                    field: "province",
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
    methods: {
        async handleConfirm(data) {
            if (data) {
                this.loading = true;
                try {
                    await updateHospital(
                        this.state.id,
                        data.name,
                        data.desc,
                        data.province,
                        data.address,
                        data.phone_num,
                        data.images.map(i => i.url),
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
