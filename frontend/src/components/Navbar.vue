<template>
  <div id="navbar">
    <b-navbar toggleable="lg" type="light" variant="light">
      <b-navbar-brand href="#">NavBar</b-navbar-brand>

      <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

      <b-collapse id="nav-collapse" is-nav>
        <b-navbar-nav>
          <b-nav-item><router-link to="/">Trang chủ</router-link></b-nav-item>
          <b-nav-item
            ><router-link to="/about">Giới thiệu</router-link></b-nav-item
          >
          <b-nav-item>Tìm kiếm bác sĩ</b-nav-item>
          <b-nav-item>Hướng dẫn khách hàng</b-nav-item>
        </b-navbar-nav>

        <!-- Right aligned nav items -->
        <b-navbar-nav class="ml-auto">
          <b-nav-form>
            <b-form-input
              size="sm"
              class="mr-sm-2"
              placeholder="Search"
            ></b-form-input>
            <!-- <b-button size="sm" class="my-2 my-sm-0" type="submit">Search</b-button> -->
            <b-button
              size="sm"
              class="my-2 my-sm-0"
              variant="primary"
              v-b-modal.modal
              ><b>Đăng ký khám</b></b-button
            >
          </b-nav-form>
        </b-navbar-nav>
      </b-collapse>
    </b-navbar>
    <b-modal
      id="modal"
      title="Đăng ký khám chữa bệnh"
      scrollable
      centered
      size="lg"
    >
      <b-form>
        <b-form-group inline label="Họ và tên" label-for="fullname">
          <b-form-input id="fullname" v-model="form.fullname" required>
          </b-form-input>
        </b-form-group>
        <b-form-group label="Ngày sinh" label-for="dateofbirth">
          <b-form-datepicker
            :max="now"
            placeholder="MM/DD/YYYY"
            :date-format-options="{
              year: 'numeric',
              month: '2-digit',
              day: '2-digit',
            }"
            id="dateofbirth"
            v-model="form.dateofbirth"
            required
          >
          </b-form-datepicker>
        </b-form-group>
        <b-form-group label="Điện thoại" label-for="phone">
          <b-form-input id="phone" v-model="form.phone" required>
          </b-form-input>
        </b-form-group>
        <b-form-group label="Email" label-for="email">
          <b-form-input id="email" v-model="form.email" required>
          </b-form-input>
        </b-form-group>
        <b-form-group label="Địa đểm gần bạn" label-for="location">
          <b-form-select
            id="location"
            v-model="form.location"
            :options="optionLocation"
            @change="getHospitals"
            required
          >
          </b-form-select>
        </b-form-group>
        <b-form-group
          label="Lựa chọn bệnh viện"
          label-for="hospital"
          v-if="form.location != null"
        >
          <b-form-select
            id="hospital"
            v-model="form.hospital"
            :options="optionHospital"
            @change="getDoctors"
            required
          >
          </b-form-select>
        </b-form-group>
        <b-form-group
          label="Lựa chọn bác sĩ"
          label-for="doctor"
          v-if="form.location != null && form.hospital != null"
        >
          <b-form-select
            id="doctor"
            v-model="form.doctor"
            :options="optionDoctor"
            required
          >
          </b-form-select>
        </b-form-group>
        <b-form-group label="Ngày khám bệnh" label-for="appointmentdate">
          <b-form-datepicker
            :min="now"
            placeholder="MM/DD/YYYY"
            :date-format-options="{
              year: 'numeric',
              month: '2-digit',
              day: '2-digit',
            }"
            id="appointmentdate"
            v-model="form.appointmentdate"
            required
          >
          </b-form-datepicker>
        </b-form-group>
        <b-form-group label="Nhu cầu khám bệnh" label-for="describe">
          <b-form-textarea id="describe" v-model="form.describe" required>
          </b-form-textarea>
        </b-form-group>
      </b-form>
      <b-card class="mt-3" header="Form Data Result">
        <pre class="m-0">{{ form }}</pre>
      </b-card>
      <template #modal-footer>
        <b-button size="sm" variant="success" @click="regis">
          Đăng ký
        </b-button>
        <b-button size="sm" variant="danger" @click="cancel()">
          Hủy bỏ
        </b-button>
      </template>
    </b-modal>
  </div>
</template>

<script>
export default {
  name: "Navbar",
  components: {},
  data() {
    return {
      now: new Date(),
      optionLocation: [],
      optionHospital: [],
      optionDoctor: [],
      form: {
        fullname: null,
        dateofbirth: null,
        phone: null,
        email: null,
        location: null,
        hospital: null,
        doctor: null,
        appointmentdate: null,
        describe: null,
      },
    };
  },
  mounted() {
    const requestOptions = {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({}),
    };
    this.optionLocation = [];
    fetch("http://13.213.43.111:8080/api/province/_search", requestOptions)
      .then((response) => response.json())
      .then((data) => {
        data.forEach((element) => {
          this.optionLocation.push({
            value: element._id.$oid,
            text: element.name,
          });
        });
      })
      .catch((error) => console.log(error));
  },
  methods: {
    regis: function () {
      if (!this.validInfor()) {
        alert("Các trường thông tin không được để trống");
      }
    },
    validInfor: function () {
      for (let key in this.form) {
        if (this.form[key] == null || this.form[key] == "") {
          return false;
        }
      }
      return true;
    },
    getHospitals: function () {
      this.optionHospital = [];
      const requestOptions = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          ids: [{ $oid: this.form.location }],
          fields: ["province"],
        }),
      };
      fetch("http://13.213.43.111:8080/api/hospital/_relate", requestOptions)
        .then((response) => response.json())
        .then((data) => {
          data.forEach((element) => {
            this.optionHospital.push({
              value: element._id.$oid,
              text: element.name,
            });
          });
        })
        .catch((error) => console.log(error));
    },
    getDoctors: function () {
      this.optionDoctor = [];
      const requestOptions = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          ids: [{ $oid: this.form.location }],
          fields: ["province"],
        }),
      };
      fetch("http://13.213.43.111:8080/api/hospital/_get", requestOptions)
        .then((response) => response.json())
        .then((data) => {
          data.forEach((element) => {
            this.optionHospital.push({
              value: element._id.$oid,
              text: element.name,
            });
          });
        })
        .catch((error) => console.log(error));
    },
  },
};
</script>