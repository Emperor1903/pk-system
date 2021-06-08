<template>
<el-dialog
  v-if="fields"
  v-loading="loading"
  top="1"  
  :title="state.title"
  :visible.sync="state.visible"
  width="50%">
  <Form :state="state" :fields="fieldList" @confirm="handleConfirm"/>
</el-dialog>
</template>

<script>
import {newDoctor, searchClinic, searchSpecialization} from "../api/index.js";

export default {
    
    name: "NewDoctorDialog",
    props: ["state"],
    components: {
        Form: () => import("./FormWithImage.vue"),
    },
    data() {
        return {
            loading: false,
            innerState: {},
            fields: {
                "name": {
                    name: "Bác sĩ",
                    type: "input",
                    value: "",
                },
                "short_intro": {
                    name: "Giới thiệu ngắn",
                    type: "area",
                    row: 2,
                    value: ""
                },
                "intro": {
                    name: "Giới thiệu",
                    type: "area",
                    row: 4,
                    value: ""
                },
                "clinic": {
                    name: "Phòng khám",
                    type: "search",
                    value: "",
                    query: async function(keyword, callback) {
                        let response = await searchClinic(keyword, null, 0);
                        let data = response.data.map(i => {
                            if(i)
                                return {
                                    value: i.name,
                                    id: i._id,
                                    field: "clinic",
                                }
                        })
                        callback(data);
                    }
                },
                "specialization": {
                    name: "Chuyên môn",
                    type: "search",
                    value: "",
                    query: async function(keyword, callback) {
                        let response = await searchSpecialization(keyword, null, 0);
                        let data = response.data.map(i => {
                            if (i)
                                return {
                                    value: i.name,
                                    id: i._id,
                                    field: "specialization",
                                }
                        });
                        callback(data);
                    }
                },
                "position": {
                    name: "Chức vụ",
                    type: "input",
                    value: "",                    
                },
                "email": {
                    name: "Email",
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
        },
    },
    mounted() {
        if (this.state.clinic) {
            this.fields.clinic.value = this.state.clinic;
        }
    },
    watch: {
        visible() {
            if (this.state.data) {
                for (var key in this.fields) {
                    this.fields[key].value = this.state.data[key];
                }
            }
        }
    },
    methods: {
        async handleConfirm(data) {
            if (data) {
                this.loading = true;
                console.log(data);
                try {
                    await newDoctor(
                        data.name,
                        data.short_intro,
                        data.intro,
                        data.clinic,                
                        data.specialization,
                        data.position,
                        data.email,
                        data.phone_num,
                        data.images.map(i => i.url)
                    );
                } finally {
                    this.loading = false;
                }
                this.$emit('confirm');
                this.state.visible = false;

                for(var key in this.fields) {
                    var type = this.fields[key].type;
                    if(type === "area" || type === "input") this.fields[key] = "";
                }
            }
        },
        
    },
}
</script>
