{% component standings() %}
{% set data = load_data(path="processed-standings.csv", format="csv") %}

<div class="row g-4">
<div class="col-12">
<div class="card">
<div class="card-header text-center">Standings</div>
<div class="card-body d-flex justify-content-center">
<div class="overflow-auto">

| Team | Sets Won | Sets Lost | Win % | Games Behind |
| ---- | -------- | --------- | ----- | ------------ |
{% for row in data.records %} {% for value in row %}| {{ value }} {% endfor %} |
{% endfor %}

</div>
</div>
</div>
</div>
</div>
{% endcomponent %}
