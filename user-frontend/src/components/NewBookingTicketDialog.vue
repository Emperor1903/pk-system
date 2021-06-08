<template>
<el-dialog
  v-loading="loading"
  :title="state.title"
  :visible.sync="state.visible">
  <div class="box">
    <div>{{ form }}</div>
    <el-steps :active="active" finish-status="success" :space="250" simple class="header">
      <el-step title="Bước 1"></el-step>
      <el-step title="Bước 2"></el-step>
      <el-step title="Bước 3"></el-step>
      <el-step title="Bước 4"></el-step>
      <el-step title="Bước 5"></el-step>
    </el-steps>
<<<<<<< HEAD
    
    <div class="form">
      <FormStep1 v-if="active == 0" :form="form"></FormStep1>
      <FormStep2 v-if="active == 1" :form="form"></FormStep2>
      <FormStep3 v-if="active == 2" :form="form"></FormStep3>
      <FormStep4 v-if="active == 3" :form="form"></FormStep4>
      <FormStep5 v-if="active == 4" :form="form"></FormStep5>
    </div>

    <span slot="footer" class="dialog-footer">
      <el-button v-if="active>0" @click="previous" type="primary">Bước trước</el-button>
      <el-button v-if="active<4"  @click="next" type="success">Bước sau</el-button>
    </span>
  </div>
</el-dialog>
=======
    <FormStep1 v-if="active == 0" :form="form"></FormStep1>
    <FormStep2 v-if="active == 1" :form="form"></FormStep2>
    <FormStep3 v-if="active == 2" :form="form"></FormStep3>
    <FormStep4 v-if="active == 3" :form="form"></FormStep4>
    <FormStep5 v-if="active == 4" :form="form"></FormStep5>
    <el-button v-if="active > 0" style="margin-top: 12px" @click="previous"
      >Bước trước</el-button
    >
    <el-button v-if="active < 4" style="margin-top: 12px" @click="next"
      >Bước sau</el-button
    >
  </el-dialog>
>>>>>>> b8a70a69d7fd114e1bfcad8b202e6e22aca78198
</template>

<style>
.box {
    width: 800px;
    height: 500px;
    margin: auto;    
}

.header {
    margin: 10px;
}

.form {
    margin: 20px;
    margin-left: 0;
}

.button-group {
    margin: auto;
}
</style>

<script>
export default {
  components: {
    FormStep1: () => import("./FormStep1"),
    FormStep2: () => import("./FormStep2"),
    FormStep3: () => import("./FormStep3"),
    FormStep4: () => import("./FormStep4"),
    FormStep5: () => import("./FormStep5"),
  },
  props: ["state"],
  data() {
    return {
      loading: false,
      active: 0,
      form: {
        name: "",
        date_of_birth: null,
        phone_num: "",
        address: "",
        gender_is_male: true,
        email: "",
        province: "",
        hospital: "",
        clinic: "",
        doctor: "",
        shift: "",
        doctorData: [],
        shiftData: [],
      },
    };
  },
  methods: {
    next() {
      this.active++;
    },
    previous() {
      this.active--;
    },
  },
};
</script>
