var gulp      = require('gulp'),
    sass      = require('gulp-sass'),
    concat    = require('gulp-concat'),
    render    = require('gulp-nunjucks-html'),
    sync      = require('browser-sync'),
    plumber   = require('gulp-plumber'),
    path      = require('path'),
    data      = require('gulp-data'),
    fs        = require('fs'),
    del       = require('del'),
    sequence = require('run-sequence')
    buildDir  = ".build";
    
gulp.task('vendor', function(){
  return gulp.src([
    './node_modules/jquery/dist/jquery.min.js',
    './node_modules/jquery-rss/dist/jquery.rss.min.js',
    './node_modules/bootstrap/dist/js/bootstrap.min.js',
    './node_modules/jquery.easing/jquery.easing.min.js',
    './node_modules/scrollreveal/dist/scrollreveal.min.js',
  ])
  .pipe(concat('vendor.js'))
  .pipe(gulp.dest(__dirname + '/' + buildDir + '/js'));
});
    
gulp.task('scss', function() {
  var options = {
    errorLogToConsole: true,
    outputStyle: 'expanded',
    includePaths: [
      __dirname + '/node_modules/bootstrap/scss'
    ]
  }
  return gulp
    .src('./src/scss/**/*.scss')
    .pipe(sass(options).on('error', sass.logError))
    .pipe(gulp.dest('./' + buildDir + '/css'))
    .pipe(sync.stream());
});

gulp.task('images', function() {
  return gulp.src('./src/images/**/*.*')
    .pipe(plumber())
    .pipe(gulp.dest('./' + buildDir + '/images'));
});

gulp.task('fonts', function() {
  return gulp.src('./src/fonts/**/*.*')
    .pipe(plumber())
    .pipe(gulp.dest('./' + buildDir + '/fonts'));
});

gulp.task('js', function() {
  return gulp.src('./src/js/**/*.js')
    .pipe(concat('app.js'))
    .pipe(gulp.dest('./' + buildDir + '/js'));
});

gulp.task('render', function() {
  return gulp.src('src/*.html')
    .pipe(render({
      searchPaths: ['src', 'src/sections']
    }))
    .pipe(gulp.dest(buildDir))
    .pipe(sync.stream());
});

gulp.task('watch', function(){
  gulp.watch('./src/scss/**/*.scss', ['scss']);
  gulp.watch("./src/**/*.html", ['render']);
  gulp.watch("./src/**/*.json", ['render']);
  gulp.watch("./src/images/**/*", ['images']);
});

gulp.task('sync', function(){
  var files = [
    buildDir + '/**/*.html',
    buildDir + '/css/*.css',
    buildDir + '/images/**/*.*',
    buildDir + '/js/**/*.js'
  ]

  sync.init(files, {
    server: {
      baseDir: './' + buildDir + '/'
    }
  })
});

gulp.task('clean', function() {
  return del(buildDir, {force: true});
});


gulp.task('default',  sequence(
  'clean',
  ['vendor', 'images', 'fonts'],
  'scss', 
  'js',
  'render',
  'watch',
  'sync'
));
