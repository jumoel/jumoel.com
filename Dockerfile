FROM ruby:2.2
WORKDIR /blog

RUN curl -sL https://deb.nodesource.com/setup_7.x | bash -
RUN apt install -y nodejs build-essential

RUN gem install bundler
ADD Gemfile .
ADD Gemfile.lock .

RUN bundle
