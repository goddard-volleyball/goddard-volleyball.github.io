+++
template = "page.html"
title = "Contact"
+++

{% set teamslink = "https://teams.microsoft.com/l/team/19%3AISKhMuuOiKKqi8XmaE8lFof5ioQDiB_vM4HloXYRF5I1%40thread.tacv2/conversations?groupId=ed532da8-60f5-4a8f-9ed0-f74df64e1f22&tenantId=7005d458-45be-48ae-8140-d43da96dd17b" %}

<div class="row g-4">
    <div class="col-12">
        <div class="card">
            <div class="card-header text-center">Contact the League</div>
            <div class="card-body mx-auto text-center">

| President | Lorinda Yam |
| --------- | ----------- |
| Vice President | Samelys Rodriguez |
| Treasurer | Samelys Rodrigues (Acting) |
| Secretary | Lorinda Yam (Acting) |
| Web Master | Karen Keadle-Calvert |
| Inquiries/Recruitment | Lorinda Yam |

</div>
</div>
</div>
</div>

<div class="row g-4 mt-0">
    <div class="col-12">
        <div class="card">
            <div class="card-header text-center">Contact the League</div>
            <div class="card-body">
                <div class="row g-4">
                    <div class="col-6">
                        <a class="btn btn-primary bg-theme text-dark button w-100" href="mailto:lorinda.s.yam@nasa.gov" role="button">Email Lorinda</a>
                    </div>
                    <div class="col-6">
                        <a class="btn btn-primary bg-theme text-dark button w-100" href="{{ teamslink }}" role="button">Microsoft Teams</a>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>


{% component teamCard(number, cap1, cap2) %}
<div class="col-12 col-md-6 col-lg-3">
    <div class="card h-100">
        <div class="card-header text-center">Team {{ number }}</div>
        <div class="card-body">{{ cap1 }}</div>
        <div class="card-body">{{ cap2 }}</div>
    </div>
</div>
{% endcomponent %}

<div class="row g-4 mt-0">
    <div class="col-12">
        <div class="card">
            <div class="card-header text-center">Team Contacts</div>
            <div class="card-body m-0">
<div class="row g-4">
{{ <teamCard number="1" cap1="Sierra Budinoff" cap2="Amy Feng" /> }}
{{ <teamCard number="2" cap1="Neal Devine" cap2="Frank Robinson" /> }}
{{ <teamCard number="3" cap1="Rob Luchessi" cap2="Carlos Lugo Acosta" /> }}
{{ <teamCard number="5" cap1="Karen Keadle-Calvert" cap2="Rachel Goldman" /> }}
</div>
<div class="row g-4 mt-0">
{{ <teamCard number="8" cap1="Lorinda Yam" cap2="Emily Bell" /> }}
{{ <teamCard number="11" cap1="Tom Grubb" cap2="Phil Mitchell" /> }}
{{ <teamCard number="13" cap1="Keith Hogie" cap2="Kate Gasaway" /> }}
{{ <teamCard number="15" cap1="Samelys Rodriguez" cap2="Evelyn Andresen" /> }}
</div>
            </div>
        </div>
    </div>
</div>