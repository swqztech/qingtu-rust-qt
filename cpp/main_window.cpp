#include "main_window.h"

#include "qingtu_ffi.h"

#include <QCheckBox>
#include <QComboBox>
#include <QFileDialog>
#include <QGroupBox>
#include <QHBoxLayout>
#include <QLabel>
#include <QLineEdit>
#include <QListWidget>
#include <QMessageBox>
#include <QPushButton>
#include <QSpinBox>
#include <QTextEdit>
#include <QVBoxLayout>

MainWindow::MainWindow(QWidget* parent) : QMainWindow(parent) {
  setWindowTitle(QStringLiteral("轻图"));
  resize(560, 720);

  auto* root = new QWidget(this);
  auto* layout = new QVBoxLayout(root);

  auto* hero = new QLabel(QStringLiteral("本地压缩 · 横向拼接"));
  hero->setStyleSheet("font-size: 18px; font-weight: bold; color: #134E4A;");
  layout->addWidget(hero);

  list_ = new QListWidget;
  layout->addWidget(list_, 1);

  auto* row = new QHBoxLayout;
  auto* btnAdd = new QPushButton(QStringLiteral("添加图片"));
  auto* btnClear = new QPushButton(QStringLiteral("清空"));
  connect(btnAdd, &QPushButton::clicked, this, &MainWindow::onAddFiles);
  connect(btnClear, &QPushButton::clicked, this, &MainWindow::onClear);
  row->addWidget(btnAdd);
  row->addWidget(btnClear);
  layout->addLayout(row);

  auto* opts = new QGroupBox(QStringLiteral("压缩选项"));
  auto* optLayout = new QVBoxLayout(opts);
  auto* qRow = new QHBoxLayout;
  qRow->addWidget(new QLabel(QStringLiteral("质量")));
  quality_ = new QSpinBox;
  quality_->setRange(1, 100);
  quality_->setValue(80);
  qRow->addWidget(quality_);
  qRow->addWidget(new QLabel(QStringLiteral("最长边 (0=不缩放)")));
  max_side_ = new QSpinBox;
  max_side_->setRange(0, 12000);
  max_side_->setValue(0);
  qRow->addWidget(max_side_);
  optLayout->addLayout(qRow);

  auto* fRow = new QHBoxLayout;
  fRow->addWidget(new QLabel(QStringLiteral("格式")));
  format_ = new QComboBox;
  format_->addItem(QStringLiteral("保持原格式"), 0);
  format_->addItem(QStringLiteral("JPEG"), 1);
  format_->addItem(QStringLiteral("PNG"), 2);
  fRow->addWidget(format_);
  optLayout->addLayout(fRow);

  custom_out_ = new QCheckBox(QStringLiteral("自定义输出目录"));
  out_dir_ = new QLineEdit;
  out_dir_->setPlaceholderText(QStringLiteral("留空则输出到各图目录下的 compressed/"));
  out_dir_->setEnabled(false);
  connect(custom_out_, &QCheckBox::toggled, out_dir_, &QLineEdit::setEnabled);
  optLayout->addWidget(custom_out_);
  optLayout->addWidget(out_dir_);
  layout->addWidget(opts);

  auto* actRow = new QHBoxLayout;
  auto* btnCompress = new QPushButton(QStringLiteral("开始压缩"));
  auto* btnStitch = new QPushButton(QStringLiteral("横向拼接"));
  btnCompress->setStyleSheet("background:#0D9488;color:white;padding:8px;");
  connect(btnCompress, &QPushButton::clicked, this, &MainWindow::onCompress);
  connect(btnStitch, &QPushButton::clicked, this, &MainWindow::onStitch);
  actRow->addWidget(btnCompress);
  actRow->addWidget(btnStitch);
  layout->addLayout(actRow);

  log_ = new QTextEdit;
  log_->setReadOnly(true);
  log_->setMinimumHeight(140);
  layout->addWidget(log_);

  setCentralWidget(root);
  setStyleSheet(
      "QMainWindow { background: #F0FDFA; }"
      "QListWidget { background: white; border: 1px solid #99F6E4; }");
}

std::vector<std::string> MainWindow::pathStrings() const {
  std::vector<std::string> v;
  v.reserve(files_.size());
  for (const auto& f : files_) {
    v.push_back(f.toStdString());
  }
  return v;
}

void MainWindow::appendLog(const QString& text) {
  log_->append(text);
}

void MainWindow::onAddFiles() {
  const auto selected = QFileDialog::getOpenFileNames(
      this, QStringLiteral("选择图片"), QString(),
      QStringLiteral("图片 (*.png *.jpg *.jpeg *.bmp *.gif *.webp)"));
  for (const auto& p : selected) {
    if (!files_.contains(p)) {
      files_.append(p);
      list_->addItem(p);
    }
  }
}

void MainWindow::onClear() {
  files_.clear();
  list_->clear();
  log_->clear();
}

void MainWindow::onCompress() {
  auto paths = pathStrings();
  if (paths.empty()) {
    QMessageBox::information(this, QStringLiteral("轻图"), QStringLiteral("请先添加图片"));
    return;
  }
  std::vector<const char*> cpaths;
  cpaths.reserve(paths.size());
  for (const auto& s : paths) {
    cpaths.push_back(s.c_str());
  }
  const bool custom = custom_out_->isChecked();
  const QByteArray out = out_dir_->text().toUtf8();
  char* msg = qingtu_compress_files(
      cpaths.data(), static_cast<int>(cpaths.size()), quality_->value(), max_side_->value(),
      format_->currentData().toInt(), custom ? 1 : 0, custom ? out.constData() : nullptr);
  if (msg) {
    appendLog(QString::fromUtf8(msg));
    qingtu_free_string(msg);
  }
}

void MainWindow::onStitch() {
  auto paths = pathStrings();
  if (paths.size() < 2) {
    QMessageBox::information(this, QStringLiteral("轻图"), QStringLiteral("拼接需要至少 2 张图"));
    return;
  }
  std::vector<const char*> cpaths;
  for (const auto& s : paths) {
    cpaths.push_back(s.c_str());
  }
  const bool custom = custom_out_->isChecked();
  const QByteArray out = out_dir_->text().toUtf8();
  char* msg = qingtu_stitch_files(cpaths.data(), static_cast<int>(cpaths.size()), custom ? 1 : 0,
                                  custom ? out.constData() : nullptr);
  if (msg) {
    appendLog(QString::fromUtf8(msg));
    qingtu_free_string(msg);
  }
}
