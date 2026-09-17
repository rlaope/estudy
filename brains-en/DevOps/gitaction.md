# GitHub Actions

## GitHub Actions
GitHub Actions is a tool that can **automate** software development workflows based on GitHub repositories.
Simply put, it can be described as a CI/CD tool provided by GitHub.

Workflows can be custom-built based on various events occurring in a GitHub repository, such as build, test, package, release, and deploy.

Workflows run in Linux, macOS, and Windows environments hosted by GitHub, which are called Runners.
Additionally, these Runners can also be run directly in environments self-hosted by the user, which are called self-hosted runners.

Workflows shared by many people can be found on the GitHub Marketplace.

## Workflow Pricing
Up to 20 workflows can be registered per repository. Each Job, which is a step within a workflow, can run for a maximum of 6 hours and will automatically stop if this limit is exceeded.
Also, depending on your GitHub account plan, the number of Jobs that can run across all your Git repositories is limited.

If you call the GitHub API within a Job, it is limited to a maximum of 1,000 calls per hour.
It is free for public repositories, while private repositories are charged after the free usage allocated to the account.

## Workflow Syntax
Workflows are written in .yml files.
```yml
name: 워크플로우의 이름

on: push # 단일 이벤트 사용

on: [ push, pull_request ] # 이벤트 목록으로 사용

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main
# 동작 유형 또는 구성과 함께 다중 이벤트 사용

on:
  release:
    types: [ published, created, edited ]
# types 키워드를 활용하면 워크플로우를 실행하는 활동의 범위를 좁힐 수 있다.
# ex) published, unpublished, created, edited, deleted or prereleased.

on:
  push:
    branches:
      - main
      - '적용할 브랜치'
      - 'releases/**'
    tags:
      - v1
      - v1.*
# 적용할 브랜치를 설정할 수 있고, 태그에도 적용할 수 있다.

on:
  push:
    branches-ignore:
      - '제외할 브랜치'
      - 'releases/**-alpha'
    tags-ignore:
      - v1.*
# 제외할 브랜치를 설정할 수 있고, 태그에도 적용할 수 있다.

on:
  push:
    branches:
      - 'releases/**'
      - '!releases/**-alpha'
# 제외할 브랜치를 !로 설정할 수도 있다.

on:
  schedule:
    - cron: '30 5,17 * * *'
#시간으로 설정할 수도 있다.

```
