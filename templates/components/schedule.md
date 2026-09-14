{% component schedule() %}
{% set data = load_data(path="processed-schedule.csv", format="csv") %}

<div class="row g-4 mt-0">
<div class="col-12">
<div class="card">
<div class="card-header text-center">Schedule</div>
<div class="card-body">
<div class="row">
<div class="col-12">
{% for row in data.records %}
<div class="card mt-4">
<div class="card-header text-center">{{ row[0] }}</div>
<div class="card-body">
<div class="row align-items-center">
<div class="col-2">6:20</div>
<div class="col-10">
<div class="row gy-2">
<div class="col-12 col-md-6">
<div class="row g-0">
<div class="col-5 p-2 bg-light text-center text-dark rounded-start-3">
Team {{ row[1] }}
</div>
<div class="col-1 p-2 border-top border-bottom"></div>
<div class="col-1 p-2 text-end border-top border-bottom"></div>
<div class="col-5 p-2 bg-theme text-center text-dark rounded-end-3">
Team {{ row[2] }}
</div>
</div> 
</div>
<div class="col-12 col-md-6">
<div class="row g-0">
<div class="col-5 p-2 bg-light text-center text-dark rounded-start-3">
Team {{ row[3] }}
</div>
<div class="col-1 p-2 border-top border-bottom"></div>
<div class="col-1 p-2 text-end border-top border-bottom"></div>
<div class="col-5 p-2 bg-theme text-center text-dark rounded-end-3">
Team {{ row[4] }}
</div>
</div> 
</div>
</div>
</div>
</div>
</div>
<div class="card-body border-top">
<div class="row align-items-center">
<div class="col-2">7:40</div>
<div class="col-10">
<div class="row gy-2">
<div class="col-12 col-md-6">
<div class="row g-0">
<div class="col-5 p-2 bg-light text-center text-dark rounded-start-3">
Team {{ row[5] }}
</div>
<div class="col-1 p-2 border-top border-bottom"></div>
<div class="col-1 p-2 text-end border-top border-bottom"></div>
<div class="col-5 p-2 bg-theme text-center text-dark rounded-end-3">
Team {{ row[6] }}
</div>
</div> 
</div>
<div class="col-12 col-md-6">
<div class="row g-0">
<div class="col-5 p-2 bg-light text-center text-dark rounded-start-3">
Team {{ row[7] }}
</div>
<div class="col-1 p-2 border-top border-bottom"></div>
<div class="col-1 p-2 text-end border-top border-bottom"></div>
<div class="col-5 p-2 bg-theme text-center text-dark rounded-end-3">
Team {{ row[8] }}
</div>
</div> 
</div>
</div>
</div>
</div>
</div>
</div>
{% endfor %}
</div>
</div>
</div>
</div>
</div>
</div
{% endcomponent %}
