+++
template = "page.html"
title = "Scores"
+++

<div class="row g-4">
    <div class="col-12">
        <div class="card">
            <div class="card-header text-center">Standings</div>
            <div class="card-body d-flex justify-content-center">
                <div class="overflow-auto">
{% standings() %}
test
{% end %}
                </div>
            </div>
        </div>
    </div>
</div>

<div class="row g-4 mt-0">
    <div class="col-12">
        <div class="card">
            <div class="card-header text-center">Schedule</div>
            <div class="card-body">
                <div class="row">
                    <div class="col-12">
{% schedule() %}
test
{% end %}                        
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
