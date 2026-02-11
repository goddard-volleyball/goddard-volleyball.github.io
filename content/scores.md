+++
template = "base.html"
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
                        <div class="card">
                            <div class="card-header text-center">10/10/26</div>
                            <div class="card-body">
                                <div class="row align-items-center">
                                    <div class="col-2">6:20</div>
                                    <div class="col-10">
                                        <div class="row gy-2">
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Chaos
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        3
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        1 
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Stooges
                                                    </div>
                                                </div> 
                                            </div>
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Blue
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Tom
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
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Sam
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        3
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        1 
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        UMD
                                                    </div>
                                                </div> 
                                            </div>
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Champions
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Condiments
                                                    </div>
                                                </div> 
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="card mt-4">
                            <div class="card-header text-center">10/17/26</div>
                            <div class="card-body">
                                <div class="row align-items-center">
                                    <div class="col-2">6:20</div>
                                    <div class="col-10">
                                        <div class="row gy-2">
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Chaos
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        3
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        1 
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Tom
                                                    </div>
                                                </div> 
                                            </div>
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Champions
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Sam
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
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Condiments
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        3
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        1 
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        UMD
                                                    </div>
                                                </div> 
                                            </div>
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Blue
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Sam
                                                    </div>
                                                </div> 
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="card mt-4">
                            <div class="card-header text-center">10/24/26</div>
                            <div class="card-body">
                                <div class="row align-items-center">
                                    <div class="col-2">6:20</div>
                                    <div class="col-10">
                                        <div class="row gy-2">
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Condiments
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        3
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        1
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        UMD
                                                    </div>
                                                </div> 
                                            </div>
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Tom
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Sam
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
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Blue
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        3
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        1
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Champions
                                                    </div>
                                                </div> 
                                            </div>
                                            <div class="col-12 col-md-6">
                                                <div class="row g-0">
                                                    <div class="col-5 p-2 bg-light text-dark rounded-start-3">
                                                        Stooges
                                                    </div>
                                                    <div class="col-1 p-2 border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-1 p-2 text-end border-top border-bottom">
                                                        2
                                                    </div>
                                                    <div class="col-5 p-2 bg-theme text-end text-dark rounded-end-3">
                                                        Condiments
                                                    </div>
                                                </div> 
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
