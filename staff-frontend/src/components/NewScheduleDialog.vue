<template>
<el-dialog
  v-if="fields"
  v-loading="loading"
  :title="state.title"
  :visible.sync="state.visible"
  width="30%">
  <Form :state="state" :fields="fieldList" @confirm="handleConfirm"/>
</el-dialog>
</template>

<script>
import {newSchedule, searchClinic, searchDoctor, getDocument} from "../api/index.js";

export default {
    
    name: "NewScheduleDialog",
    props: ["state"],
    components: {
        Form: () => import("./Form.vue"),
    },
    data() {
        return {
            loading: false,
            innerState: {},
            fields: {
                "doctor": {
                    name: "Bác sĩ",
                    type: "search",
                    value: "",
                    query: async function(keyword, callback) {
                        let response = await searchDoctor(keyword, null, 0);
                        let data = response.data.map(i => {
                            if(i)
                                return {
                                    value: i.name,
                                    id: i._id,
                                    field: "doctor",
                                }
                        })
                        callback(data);
                    }
                },
                "shift_num": {
                    name: "Ca số",
                    type: "option",
                    value: 0,
                    options: [
                        {
                            value: 0,
                            label: "ca 1: 8h - 12h"
                        },
                        {
                            value: 1,
                            label: "ca 2: 13h - 17h"
                        },
                    ]
                },
                "shift_day": {
                    name: "Vào thứ",
                    type: "option",
                    value: 0,
                    options: [
                        {
                            value: 0,
                            label: "Thứ 2"
                        },
                        {
                            value: 1,
                            label: "Thứ 3"
                        },
                        {
                            value: 2,
                            label: "Thứ 4"
                        },
                        {
                            value: 3,
                            label: "Thứ 5"
                        },
                        {
                            value: 4,
                            label: "Thứ 6"
                        },
                        {
                            value: 5,
                            label: "Thứ 7"
                        },
                        {
                            value: 6,
                            label: "Chủ nhật"
                        },
                    ]
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
        if (this.state.doctor) {
            this.fields.doctor.value = this.state.doctor;
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
                let res = await getDocument("doctor", data.doctor);
                try {
                    await newSchedule(
                        data.doctor,
                        res.clinic,
                        data.shift_num,
                        data.shift_day,
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
