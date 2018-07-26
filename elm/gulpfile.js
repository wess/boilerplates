var gulp      = require('gulp'),
    sass      = require('gulp-sass'),
    concat    = require('gulp-concat'),
    sync      = require('browser-sync'),
    plumber   = require('gulp-plumber'),
    path      = require('path'),
    data      = require('gulp-data'),
    fs        = require('fs'),
    del       = require('del'),
    sequence = require('run-sequence'),
    elm       = require('gulp-elm'),
    buildDir  = "_build";
    

gulp.task('elm-init', elm.init);

gulp.task('elm', ['elm-init'], function(){
  return gulp.src('src/app/**/*.elm')
    .pipe(elm.bundle('app.js'))
    .pipe(gulp.dest(__dirname + '/' + buildDir + '/js'));
});

gulp.task('copymaps', function() {
  return gulp.src('./node_modules/bootstrap/dist/js/bootstrap.min.js.map')
  .pipe(gulp.dest(buildDir + '/js'));
});

gulp.task('vendor', ['copymaps'], function(){
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

gulp.task('images', function() {
  return gulp.src('./src/images/**/*.*')
    .pipe(plumber())
    .pipe(gulp.dest('./' + buildDir + '/images'));
});

gulp.task('copy', ['images'], function(){
  return gulp.src([
    "src/index.html",
    "src/js/**/*",
    "src/images"
  ])
  .pipe(gulp.dest(buildDir));
});

gulp.task('watch', function(){
  gulp.watch('./src/scss/**/*.scss',  ['scss']);
  gulp.watch("./src/images/**/*",     ['images']);
  gulp.watch("./src/**/*.elm",        ['elm']);
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
  return del([buildDir + '/*', '!' + buildDir + '/.gitkeep'], {force: true});
});

gulp.task('default',  sequence(
  'clean',
  ['vendor', 'fonts'],
  'scss', 
  'js',
  'elm',
  'copy', 
  'watch',
  'sync'
));
