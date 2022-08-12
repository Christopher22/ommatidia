#include <opencv2/imgproc/imgproc.hpp>
#include <opencv2/core/core.hpp>

#include <iostream>
#include <fstream>

struct ellipse_out {
  cv::RotatedRect eli;
  bool valid;
};

// FITTER

class ELIFITTER {
 private:
  cv::Mat CNM1 = cv::Mat::zeros(100, 100, CV_32FC1);
  cv::Mat CNM2 = cv::Mat::zeros(100, 100, CV_32FC1);

  int GL_R_STA_ELI;
  int GL_R_STE_ELI;

  int GL_SZ_R_ELI;
  int GL_SZ_O_ELI;
  int GL_SZ_D_ELI;

  float **idx_ppx_ELI;
  float **idx_ppy_ELI;
  float ***idx_px_ELI;
  float ***idx_py_ELI;
  float ***idx_nx_ELI;
  float ***idx_ny_ELI;

  bool **idx_used_ELI;
  float **idx_weight_ELI;

  std::vector<int> rad_idx;
  std::vector<cv::Point2i> rads_to_idx;

  void m_init_ELI();

 public:
  void set_INPUT_SIZE(int SZX, int SZY);

  void m_store(std::string path);

  void m_load(std::string path);

  void m_init_without_file_eli(int min_rad, int max_rad, int step_rad,
                               float step_ori, int dist_diff);

  void m_clear_eli();

  void m_fast_eli(cv::Mat &in, ellipse_out &pos);
};

class BORE {
 private:
  ELIFITTER FITTER;

  int START_Y = 10;
  int STOP_Y = 10;
  int START_X = 10;
  int STOP_X = 10;

  int GL_R_STA;
  int GL_R_STE;

  int GL_SZ_R;
  int GL_SZ_O;
  int GL_SZ_D;

  float **idx_ppx;
  float **idx_ppy;
  float ***idx_px;
  float ***idx_py;
  float ***idx_nx;
  float ***idx_ny;

  bool **idx_used;
  float **idx_weight;

  cv::Mat CNM1 = cv::Mat::zeros(100, 100, CV_32FC1);
  cv::Mat CNM2 = cv::Mat::zeros(100, 100, CV_32FC1);

  std::vector<std::vector<cv::Point2i>> INDEXES_P;
  std::vector<std::vector<cv::Point2i>> INDEXES_N;
  std::vector<std::vector<cv::Point2i>> INDEXES_O;

  void m_init();

  void m_fast_idx(cv::Mat &in);

  void m_fast(cv::Mat &in, int st_r, int en_r);

  void m_fast_train(cv::Mat &in, int st_r, int en_r, cv::RotatedRect el,
                    float dist_weight_train);

 public:
  void set_AOI(int start_x, int stop_x, int start_y,
               int stop_y);  // start is added to 0 and stop is substracted
                             // fromthe input size

  void set_INPUT_SIZE(int SZX, int SZY);

  void m_init_without_file(int min_rad, int max_rad, int step_rad,
                           float step_ori, int dist_diff);

  ellipse_out run_fast(cv::Mat input);

  ellipse_out run_fast_train(cv::Mat input, ellipse_out el,
                             float dist_weight_train);

  void cmp_indexes();

  ellipse_out run_fast_idx(cv::Mat input);

  void m_norm_weight(float prc);

  void m_clear();

  void m_store(std::string path);

  void m_load(std::string path);

  ellipse_out run_fast_iterations(cv::Mat input, int iter);

  ellipse_out run_fast_train_iterations(cv::Mat input, ellipse_out el,
                                        float dist_weight_train, int iter);

  ellipse_out run_fast_idx_iterations(cv::Mat input, int iter);
};
