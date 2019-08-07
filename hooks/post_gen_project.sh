#!/bin/sh

set -ex

git init
git remote add origin git@github.com:{{ cookiecutter.github_org }}/{{ cookiecutter.github_project }}.git
git config user.name "{{ cookiecutter.full_name }}"
git config user.email "{{ cookiecutter.email }}"
